use serde::{Serialize, Deserialize};

use crate::contracts;
use crate::types::TxRef;
use crate::TransactionStatus;
use crate::contracts::AccountIdWrapper;

use crate::std::collections::BTreeMap;
use crate::std::string::String;
use crate::std::string::ToString;

use base64;

/// SecretNote contract states.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SecretB64Code {
    b64code: BTreeMap<AccountIdWrapper, String>,
}

/// The commands that the contract accepts from the blockchain. Also called transactions.
/// Commands are supposed to update the states of the contract.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    /// Set the note for current user
    SetB64Code {
        b64code: String,
    },
}

/// The errors that the contract could throw for some queries
#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    NotAuthorized,
}

/// Query requests. The end users can only query the contract states by sending requests.
/// Queries are not supposed to write to the contract states.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    /// Read the note for current user
    DecodeB64Code,
}

/// Query responses.
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    /// Return the note for current user
    DecodeB64Code {
        decnote: String,
    },
    /// Something wrong happened
    Error(Error)
}


impl SecretB64Code {
    /// Initializes the contract
    pub fn new() -> Self {
        Default::default()
    }
}

impl contracts::Contract<Command, Request, Response> for SecretB64Code {
    // Returns the contract id
    fn id(&self) -> contracts::ContractId { contracts::SECRETB64CODE }

    // Handles the commands from transactions on the blockchain. This method doesn't respond.
    fn handle_command(&mut self, _origin: &chain::AccountId, _txref: &TxRef, cmd: Command) -> TransactionStatus {
        match cmd {
            // Handle the `SetB64Code` command with one parameter
            Command::SetB64Code { b64code } => {
                // Simply increment the counter by some value
                let current_user = AccountIdWrapper(_origin.clone());
                // Insert the note, we only keep the latest note
                self.b64code.insert(current_user, b64code);
                // Returns TransactionStatus::Ok to indicate a successful transaction
                TransactionStatus::Ok
            },
        }
    }

    // Handles a direct query and responds to the query. It shouldn't modify the contract states.
    fn handle_query(&mut self, _origin: Option<&chain::AccountId>, req: Request) -> Response {
        let inner = || -> Result<Response, Error> {
            match req {
                // Handle the `DecodeB64Code` request
                Request::DecodeB64Code => {
                    // Unwrap the current user account
                    if let Some(account) = _origin {
                        let current_user = AccountIdWrapper(account.clone());
                        if self.b64code.contains_key(&current_user) {
                            // Respond with the note in the notes
                            let b64code = self.b64code.get(&current_user).unwrap();
                            // let b64codebytes = b64code.as_bytes();
                            let decmsg = base64::decode(&b64code).unwrap();

                            let decmsgstr = match std::str::from_utf8(&decmsg) {
                                Ok(v) => v,
                                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                            };

                            return Ok(Response::DecodeB64Code { decnote: decmsgstr.to_string() })


                        }
                    }

                    // Respond NotAuthorized when no account is specified
                    Err(Error::NotAuthorized)
                },
            }
        };
        match inner() {
            Err(error) => Response::Error(error),
            Ok(resp) => resp
        }
    }
}