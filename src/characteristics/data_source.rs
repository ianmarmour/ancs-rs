pub use crate::attributes::attribute::*;
pub use crate::attributes::command::*;

use nom::{
    bytes::complete::take_until,
    multi::{many0},
    number::complete::{le_u8, le_u32},
    IResult,
};

pub const DATA_SOURCE_UUID: &str = "22EAC6E9-24D6-4BB5-BE44-B36ACE7C7BFB";

#[derive(Debug, PartialEq, Clone)]
pub struct GetNotificationAttributesResponse {
    pub command_id: CommandID,
    pub notification_uid: u32,
    pub attribute_list: Vec<Attribute>,
}

impl From<GetNotificationAttributesResponse> for Vec<u8> {
    fn from(original: GetNotificationAttributesResponse) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();

        // Convert all attributes to bytes
        let command_id: u8 = original.command_id.into();
        let app_identifier: [u8; 4] = original.notification_uid.to_le_bytes();
        let mut attribute_ids: Vec<u8> = original
            .attribute_list
            .into_iter()
            .map(|attribute| attribute.into())
            .into_iter()
            .flat_map(|att: Vec<u8>| att)
            .collect();

        vec.push(command_id);
        vec.append(&mut app_identifier.into());
        vec.append(&mut attribute_ids);

        return vec;
    }
}

impl GetNotificationAttributesResponse {
    pub fn parse(i: &[u8]) -> IResult<&[u8], GetNotificationAttributesResponse> {
        let (i, command_id) = le_u8(i)?;
        let (i, notification_uid) = le_u32(i)?;
        let (i, attribute_list) = many0(Attribute::parse)(i)?;

        Ok((
            i,
            GetNotificationAttributesResponse {
                command_id: CommandID::try_from(command_id).unwrap(),
                notification_uid: notification_uid,
                attribute_list: attribute_list,
            },
        ))
    }
}

struct GetAppAttributesRequest {
    command_id: CommandID,
    app_identifier: String,
    attribute_ids: Vec<AttributeID>,
}

impl From<GetAppAttributesRequest> for Vec<u8> {
    fn from(original: GetAppAttributesRequest) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();

        // Convert all attributes to bytes
        let command_id: u8 = original.command_id.into();
        let mut app_identifier: Vec<u8> = original.app_identifier.into_bytes();
        let mut attribute_ids: Vec<u8> = original
            .attribute_ids
            .into_iter()
            .map(|id| id.into())
            .collect();

        vec.push(command_id);
        vec.append(&mut app_identifier);
        vec.append(&mut attribute_ids);

        return vec;
    }
}

struct GetAppAttributesResponse {
    command_id: CommandID,
    app_identifier: String,
    attribute_list: Vec<Attribute>,
}

impl GetAppAttributesResponse {
    pub fn parse(i: &[u8]) -> IResult<&[u8], GetAppAttributesResponse> {
        let (i, command_id) = le_u8(i)?;
        let (i, app_identifier) = take_until(" ")(i)?;
        let (i, attribute_list) = many0(Attribute::parse)(i)?;

        Ok((
            i,
            GetAppAttributesResponse {
                command_id: CommandID::try_from(command_id).unwrap(),
                app_identifier: String::from_utf8(app_identifier.to_vec()).unwrap(),
                attribute_list: attribute_list,
            },
        ))
    }
}
