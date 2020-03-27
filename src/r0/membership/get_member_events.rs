//! [GET /_matrix/client/r0/rooms/{roomId}/members](https://matrix.org/docs/spec/client_server/r0.4.0.html#get-matrix-client-r0-rooms-roomid-members)

use ruma_api::ruma_api;
use ruma_events::{room::member::MemberEvent, EventResult};
use ruma_identifiers::RoomId;
use serde::{Deserialize, Serialize};

ruma_api! {
    metadata {
        description: "Get membership events for a room.",
        method: GET,
        name: "get_member_events",
        path: "/_matrix/client/r0/rooms/:room_id/members",
        rate_limited: false,
        requires_authentication: true,
    }

    request {
        /// The room to get the member events for.
        #[ruma_api(path)]
        pub room_id: RoomId,

        /// The point in time (pagination token) to return members for in the room. This token can
        /// be obtained from a prev_batch token returned for each room by the sync API.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[ruma_api(query)]
        pub at: Option<String>,

        /// The kind of membership to filter for. Defaults to no filtering if unspecified. When
        /// specified alongside not_membership, the two parameters create an 'or' condition: either
        /// the membership is the same as membership or is not the same as not_membership.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[ruma_api(query)]
        pub membership: Option<MembershipFilter>,

        /// The kind of memberships to *exclude* from the results. Defaults to no filtering if
        /// unspecified.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[ruma_api(query)]
        pub not_membership: Option<MembershipFilter>,
    }

    response {
        /// A list of member events.
        #[wrap_incoming(MemberEvent with EventResult)]
        pub chunk: Vec<MemberEvent>
    }

    error: crate::Error
}

/// The kind of membership state of a user to filter for.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum MembershipFilter {
    /// The user has joined.
    #[serde(rename = "join")]
    Join,

    /// The user has been invited.
    #[serde(rename = "invite")]
    Invite,

    /// The user has left.
    #[serde(rename = "leave")]
    Leave,

    /// The user has is banned.
    #[serde(rename = "ban")]
    Ban,
}
