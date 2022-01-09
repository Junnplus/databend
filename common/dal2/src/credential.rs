// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Debug, Clone)]
pub enum Credential {
    /// Basic refers to HTTP Basic Authentication.
    Basic { username: String, password: String },
    /// HMAC, also known as Access Key/Secret Key authentication.
    ///
    /// ## NOTE
    ///
    /// HMAC is just a common step of ak/sk authentication. And it's not the correct name for
    /// this type of authentication. But it's widely used and no ambiguities with other types.
    /// So we use it here to avoid using AkSk as a credential type.
    HMAC {
        access_key_id: String,
        secret_access_key: String,
    },
    /// Token refers to static API token.
    Token(String),
}

impl Credential {
    pub fn basic(username: String, password: String) -> Credential {
        Credential::Basic { username, password }
    }

    pub fn hmac(access_key_id: String, secret_access_key: String) -> Credential {
        Credential::HMAC {
            access_key_id,
            secret_access_key,
        }
    }

    pub fn token(token: String) -> Credential {
        Credential::Token(token)
    }
}