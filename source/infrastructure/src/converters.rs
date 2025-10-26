use nimbus_auth_application::use_cases::{AccessTokenDto, UserClaimsDto};
use nimbus_auth_proto::proto::nimbus::{
    auth::entities::v1::AccessTokenProto, entities::user::v1::UserProto,
};

pub fn convert_user_into_proto(user: UserClaimsDto) -> UserProto {
    UserProto {
        id: user.id,
        user_name: user.name,
    }
}

pub fn convert_access_token_into_proto(access_token: AccessTokenDto) -> AccessTokenProto {
    AccessTokenProto {
        token: access_token.signed_access_token.to_string(),
        expires_at_unix_timestamp: access_token.signed_access_token_expires_at_unix_timestamp,
    }
}
