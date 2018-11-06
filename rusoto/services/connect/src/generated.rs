// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateUserRequest {
    /// <p>The unique identifier for the user account in the directory service directory used for identity management. If Amazon Connect is unable to access the existing directory, you can use the <code>DirectoryUserId</code> to authenticate users. If you include the parameter, it is assumed that Amazon Connect cannot access the directory. If the parameter is not included, the UserIdentityInfo is used to authenticate users from your existing directory.</p> <p>This parameter is required if you are using an existing directory for identity management in Amazon Connect when Amazon Connect cannot access your directory to authenticate users. If you are using SAML for identity management and include this parameter, an <code>InvalidRequestException</code> is returned.</p>
    #[serde(rename = "DirectoryUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_user_id: Option<String>,
    /// <p>The unique identifier for the hierarchy group to assign to the user created.</p>
    #[serde(rename = "HierarchyGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
    /// <p>Information about the user, including email address, first name, and last name.</p>
    #[serde(rename = "IdentityInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_info: Option<UserIdentityInfo>,
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The password for the user account to create. This is required if you are using Amazon Connect for identity management. If you are using SAML for identity management and include this parameter, an <code>InvalidRequestException</code> is returned.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>Specifies the phone settings for the user, including AfterContactWorkTimeLimit, AutoAccept, DeskPhoneNumber, and PhoneType.</p>
    #[serde(rename = "PhoneConfig")]
    pub phone_config: UserPhoneConfig,
    /// <p>The unique identifier for the routing profile to assign to the user created.</p>
    #[serde(rename = "RoutingProfileId")]
    pub routing_profile_id: String,
    /// <p>The unique identifier of the security profile to assign to the user created.</p>
    #[serde(rename = "SecurityProfileIds")]
    pub security_profile_ids: Vec<String>,
    /// <p>The user name in Amazon Connect for the user to create.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateUserResponse {
    /// <p>The Amazon Resource Name (ARN) of the user account created.</p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
    /// <p>The unique identifier for the user account in Amazon Connect</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// <p>The credentials to use for federation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Credentials {
    /// <p>An access token generated for a federated user to access Amazon Connect</p>
    #[serde(rename = "AccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// <p>A token generated with an expiration time for the session a user is logged in to Amazon Connect</p>
    #[serde(rename = "AccessTokenExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_expiration: Option<f64>,
    /// <p>Renews a token generated for a user to access the Amazon Connect instance.</p>
    #[serde(rename = "RefreshToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// <p>Renews the expiration timer for a generated token.</p>
    #[serde(rename = "RefreshTokenExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_expiration: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUserRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The unique identifier of the user to delete.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeUserHierarchyGroupRequest {
    /// <p>The identifier for the hierarchy group to return.</p>
    #[serde(rename = "HierarchyGroupId")]
    pub hierarchy_group_id: String,
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeUserHierarchyGroupResponse {
    /// <p>Returns a <code>HierarchyGroup</code> object.</p>
    #[serde(rename = "HierarchyGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group: Option<HierarchyGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeUserHierarchyStructureRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeUserHierarchyStructureResponse {
    /// <p>A <code>HierarchyStructure</code> object.</p>
    #[serde(rename = "HierarchyStructure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_structure: Option<HierarchyStructure>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeUserRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>Unique identifier for the user account to return.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeUserResponse {
    /// <p>A <code>User</code> object that contains information about the user account and configuration settings.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFederationTokenRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetFederationTokenResponse {
    /// <p>The credentials to use for federation.</p>
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}

/// <p>A <code>HierarchyGroup</code> object that contains information about a hierarchy group in your Amazon Connect instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HierarchyGroup {
    /// <p>The Amazon Resource Name (ARN) for the hierarchy group.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A <code>HierarchyPath</code> object that contains information about the levels in the hierarchy group.</p>
    #[serde(rename = "HierarchyPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_path: Option<HierarchyPath>,
    /// <p>The identifier for the hierarchy group.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The identifier for the level in the hierarchy group.</p>
    #[serde(rename = "LevelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_id: Option<String>,
    /// <p>The name of the hierarchy group in your instance.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A <code>HierarchyGroupSummary</code> object that contains information about the hierarchy group, including ARN, Id, and Name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HierarchyGroupSummary {
    /// <p>The ARN for the hierarchy group.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the hierarchy group.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the hierarchy group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A <code>HierarchyLevel</code> object that contains information about the levels in a hierarchy group, including ARN, Id, and Name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HierarchyLevel {
    /// <p>The ARN for the hierarchy group level.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier for the hierarchy group level.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the hierarchy group level.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A <code>HierarchyPath</code> object that contains information about the levels of the hierarchy group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HierarchyPath {
    /// <p>A <code>HierarchyGroupSummary</code> object that contains information about the level of the hierarchy group, including ARN, Id, and Name.</p>
    #[serde(rename = "LevelFive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_five: Option<HierarchyGroupSummary>,
    /// <p>A <code>HierarchyGroupSummary</code> object that contains information about the level of the hierarchy group, including ARN, Id, and Name.</p>
    #[serde(rename = "LevelFour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_four: Option<HierarchyGroupSummary>,
    /// <p>A <code>HierarchyGroupSummary</code> object that contains information about the level of the hierarchy group, including ARN, Id, and Name.</p>
    #[serde(rename = "LevelOne")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_one: Option<HierarchyGroupSummary>,
    /// <p>A <code>HierarchyGroupSummary</code> object that contains information about the level of the hierarchy group, including ARN, Id, and Name.</p>
    #[serde(rename = "LevelThree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_three: Option<HierarchyGroupSummary>,
    /// <p>A <code>HierarchyGroupSummary</code> object that contains information about the level of the hierarchy group, including ARN, Id, and Name.</p>
    #[serde(rename = "LevelTwo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_two: Option<HierarchyGroupSummary>,
}

/// <p>A <code>HierarchyStructure</code> object that contains information about the hierarchy group structure.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HierarchyStructure {
    /// <p>A <code>HierarchyLevel</code> object that contains information about the hierarchy group level.</p>
    #[serde(rename = "LevelFive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_five: Option<HierarchyLevel>,
    /// <p>A <code>HierarchyLevel</code> object that contains information about the hierarchy group level.</p>
    #[serde(rename = "LevelFour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_four: Option<HierarchyLevel>,
    /// <p>A <code>HierarchyLevel</code> object that contains information about the hierarchy group level.</p>
    #[serde(rename = "LevelOne")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_one: Option<HierarchyLevel>,
    /// <p>A <code>HierarchyLevel</code> object that contains information about the hierarchy group level.</p>
    #[serde(rename = "LevelThree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_three: Option<HierarchyLevel>,
    /// <p>A <code>HierarchyLevel</code> object that contains information about the hierarchy group level.</p>
    #[serde(rename = "LevelTwo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_two: Option<HierarchyLevel>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRoutingProfilesRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of routing profiles to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListRoutingProfilesResponse {
    /// <p>A string returned in the response. Use the value returned in the response as the value of the NextToken in a subsequent request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>RoutingProfileSummary</code> objects that include the ARN, Id, and Name of the routing profile.</p>
    #[serde(rename = "RoutingProfileSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_summary_list: Option<Vec<RoutingProfileSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSecurityProfilesRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of security profiles to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListSecurityProfilesResponse {
    /// <p>A string returned in the response. Use the value returned in the response as the value of the NextToken in a subsequent request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>SecurityProfileSummary</code> objects.</p>
    #[serde(rename = "SecurityProfileSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_summary_list: Option<Vec<SecurityProfileSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListUserHierarchyGroupsRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of hierarchy groups to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListUserHierarchyGroupsResponse {
    /// <p>A string returned in the response. Use the value returned in the response as the value of the NextToken in a subsequent request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>HierarchyGroupSummary</code> objects.</p>
    #[serde(rename = "UserHierarchyGroupSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_hierarchy_group_summary_list: Option<Vec<HierarchyGroupSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListUsersRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListUsersResponse {
    /// <p>A string returned in the response. Use the value returned in the response as the value of the NextToken in a subsequent request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>UserSummary</code> objects that contain information about the users in your instance.</p>
    #[serde(rename = "UserSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_summary_list: Option<Vec<UserSummary>>,
}

/// <p>A <code>RoutingProfileSummary</code> object that contains information about a routing profile, including ARN, Id, and Name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RoutingProfileSummary {
    /// <p>The ARN of the routing profile.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the routing profile.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the routing profile.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A <code>SecurityProfileSummary</code> object that contains information about a security profile, including ARN, Id, Name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SecurityProfileSummary {
    /// <p>The ARN of the security profile.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the security profile.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the security profile.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartOutboundVoiceContactRequest {
    /// <p>Specify a custom key-value pair using an attribute map. The attributes are standard Amazon Connect attributes, and can be accessed in contact flows just like any other contact attributes.</p> <p>There can be up to 32,768 UTF-8 bytes across all key-value pairs. Attribute keys can include only alphanumeric, dash, and underscore characters.</p> <p>For example, if you want play a greeting when the customer answers the call, you can pass the customer name in attributes similar to the following:</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. The token is valid for 7 days after creation. If a contact is already started, the contact ID is returned. If the contact is disconnected, a new contact is started.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The identifier for the contact flow to connect the outbound call to.</p> <p>To find the <code>ContactFlowId</code>, open the contact flow you want to use in the Amazon Connect contact flow editor. The ID for the contact flow is displayed in the address bar as part of the URL. For example, the contact flow ID is the set of characters at the end of the URL, after 'contact-flow/' such as <code>78ea8fd5-2659-4f2b-b528-699760ccfc1b</code>.</p>
    #[serde(rename = "ContactFlowId")]
    pub contact_flow_id: String,
    /// <p>The phone number of the customer in E.164 format.</p>
    #[serde(rename = "DestinationPhoneNumber")]
    pub destination_phone_number: String,
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The queue to add the call to. If you specify a queue, the phone displayed for caller ID is the phone number specified in the queue. If you do not specify a queue, the queue used will be the queue defined in the contact flow.</p> <p>To find the <code>QueueId</code>, open the queue you want to use in the Amazon Connect Queue editor. The ID for the queue is displayed in the address bar as part of the URL. For example, the queue ID is the set of characters at the end of the URL, after 'queue/' such as <code>queue/aeg40574-2d01-51c3-73d6-bf8624d2168c</code>.</p>
    #[serde(rename = "QueueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    /// <p>The phone number, in E.164 format, associated with your Amazon Connect instance to use for the outbound call.</p>
    #[serde(rename = "SourcePhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_phone_number: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartOutboundVoiceContactResponse {
    /// <p>The unique identifier of this contact within your Amazon Connect instance.</p>
    #[serde(rename = "ContactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopContactRequest {
    /// <p>The unique identifier of the contact to end.</p>
    #[serde(rename = "ContactId")]
    pub contact_id: String,
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopContactResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserHierarchyRequest {
    /// <p>The identifier for the hierarchy group to assign to the user.</p>
    #[serde(rename = "HierarchyGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier of the user account to assign the hierarchy group to.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserIdentityInfoRequest {
    /// <p>A <code>UserIdentityInfo</code> object.</p>
    #[serde(rename = "IdentityInfo")]
    pub identity_info: UserIdentityInfo,
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier for the user account to update identity information for.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserPhoneConfigRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>A <code>UserPhoneConfig</code> object that contains settings for <code>AfterContactWorkTimeLimit</code>, <code>AutoAccept</code>, <code>DeskPhoneNumber</code>, and <code>PhoneType</code> to assign to the user.</p>
    #[serde(rename = "PhoneConfig")]
    pub phone_config: UserPhoneConfig,
    /// <p>The identifier for the user account to change phone settings for.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserRoutingProfileRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier of the routing profile to assign to the user.</p>
    #[serde(rename = "RoutingProfileId")]
    pub routing_profile_id: String,
    /// <p>The identifier for the user account to assign the routing profile to.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserSecurityProfilesRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifiers for the security profiles to assign to the user.</p>
    #[serde(rename = "SecurityProfileIds")]
    pub security_profile_ids: Vec<String>,
    /// <p>The identifier of the user account to assign the security profiles.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

/// <p>A <code>User</code> object that contains information about a user account in your Amazon Connect instance, including configuration settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct User {
    /// <p>The ARN of the user account.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The directory Id for the user account in the existing directory used for identity management.</p>
    #[serde(rename = "DirectoryUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_user_id: Option<String>,
    /// <p>The identifier for the hierarchy group assigned to the user.</p>
    #[serde(rename = "HierarchyGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A <code>UserIdentityInfo</code> object.</p>
    #[serde(rename = "IdentityInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_info: Option<UserIdentityInfo>,
    /// <p>A <code>UserPhoneConfig</code> object.</p>
    #[serde(rename = "PhoneConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_config: Option<UserPhoneConfig>,
    /// <p>The identifier of the routing profile assigned to the user.</p>
    #[serde(rename = "RoutingProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_id: Option<String>,
    /// <p>The identifier(s) for the security profile assigned to the user.</p>
    #[serde(rename = "SecurityProfileIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_ids: Option<Vec<String>>,
    /// <p>The user name assigned to the user account.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>A <code>UserIdentityInfo</code> object that contains information about the user's identity, including email address, first name, and last name.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserIdentityInfo {
    /// <p>The email address added to the user account. If you are using SAML for identity management and include this parameter, an <code>InvalidRequestException</code> is returned.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The first name used in the user account. This is required if you are using Amazon Connect or SAML for identity management.</p>
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// <p>The last name used in the user account. This is required if you are using Amazon Connect or SAML for identity management.</p>
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

/// <p>A <code>UserPhoneConfig</code> object that contains information about the user phone configuration settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPhoneConfig {
    /// <p>The After Call Work (ACW) timeout setting, in seconds, for the user.</p>
    #[serde(rename = "AfterContactWorkTimeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_contact_work_time_limit: Option<i64>,
    /// <p>The Auto accept setting for the user, Yes or No.</p>
    #[serde(rename = "AutoAccept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_accept: Option<bool>,
    /// <p>The phone number for the user's desk phone.</p>
    #[serde(rename = "DeskPhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desk_phone_number: Option<String>,
    /// <p>The phone type selected for the user, either Soft phone or Desk phone.</p>
    #[serde(rename = "PhoneType")]
    pub phone_type: String,
}

/// <p>A <code>UserSummary</code> object that contains Information about a user, including ARN, Id, and user name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UserSummary {
    /// <p>The ARN for the user account.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier for the user account.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The Amazon Connect user name for the user account.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// Errors returned by CreateUser
#[derive(Debug, PartialEq)]
pub enum CreateUserError {
    /// <p>A resource with that name already exisits.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The limit exceeded the maximum allowed active calls in a queue.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateUserError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateUserError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "DuplicateResourceException" => {
                    return CreateUserError::DuplicateResource(String::from(error_message))
                }
                "InternalServiceException" => {
                    return CreateUserError::InternalService(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return CreateUserError::InvalidParameter(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return CreateUserError::InvalidRequest(String::from(error_message))
                }
                "LimitExceededException" => {
                    return CreateUserError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return CreateUserError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return CreateUserError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateUserError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateUserError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateUserError {
    fn from(err: serde_json::error::Error) -> CreateUserError {
        CreateUserError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateUserError {
    fn from(err: CredentialsError) -> CreateUserError {
        CreateUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateUserError {
    fn from(err: HttpDispatchError) -> CreateUserError {
        CreateUserError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateUserError {
    fn from(err: io::Error) -> CreateUserError {
        CreateUserError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUserError {
    fn description(&self) -> &str {
        match *self {
            CreateUserError::DuplicateResource(ref cause) => cause,
            CreateUserError::InternalService(ref cause) => cause,
            CreateUserError::InvalidParameter(ref cause) => cause,
            CreateUserError::InvalidRequest(ref cause) => cause,
            CreateUserError::LimitExceeded(ref cause) => cause,
            CreateUserError::ResourceNotFound(ref cause) => cause,
            CreateUserError::Throttling(ref cause) => cause,
            CreateUserError::Validation(ref cause) => cause,
            CreateUserError::Credentials(ref err) => err.description(),
            CreateUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateUserError::ParseError(ref cause) => cause,
            CreateUserError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteUserError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteUserError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalServiceException" => {
                    return DeleteUserError::InternalService(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return DeleteUserError::InvalidParameter(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return DeleteUserError::InvalidRequest(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return DeleteUserError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return DeleteUserError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteUserError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteUserError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteUserError {
    fn from(err: serde_json::error::Error) -> DeleteUserError {
        DeleteUserError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteUserError {
    fn from(err: CredentialsError) -> DeleteUserError {
        DeleteUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteUserError {
    fn from(err: HttpDispatchError) -> DeleteUserError {
        DeleteUserError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteUserError {
    fn from(err: io::Error) -> DeleteUserError {
        DeleteUserError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserError::InternalService(ref cause) => cause,
            DeleteUserError::InvalidParameter(ref cause) => cause,
            DeleteUserError::InvalidRequest(ref cause) => cause,
            DeleteUserError::ResourceNotFound(ref cause) => cause,
            DeleteUserError::Throttling(ref cause) => cause,
            DeleteUserError::Validation(ref cause) => cause,
            DeleteUserError::Credentials(ref err) => err.description(),
            DeleteUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteUserError::ParseError(ref cause) => cause,
            DeleteUserError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeUser
#[derive(Debug, PartialEq)]
pub enum DescribeUserError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeUserError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeUserError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalServiceException" => {
                    return DescribeUserError::InternalService(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return DescribeUserError::InvalidParameter(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return DescribeUserError::InvalidRequest(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return DescribeUserError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return DescribeUserError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeUserError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeUserError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeUserError {
    fn from(err: serde_json::error::Error) -> DescribeUserError {
        DescribeUserError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeUserError {
    fn from(err: CredentialsError) -> DescribeUserError {
        DescribeUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeUserError {
    fn from(err: HttpDispatchError) -> DescribeUserError {
        DescribeUserError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeUserError {
    fn from(err: io::Error) -> DescribeUserError {
        DescribeUserError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUserError {
    fn description(&self) -> &str {
        match *self {
            DescribeUserError::InternalService(ref cause) => cause,
            DescribeUserError::InvalidParameter(ref cause) => cause,
            DescribeUserError::InvalidRequest(ref cause) => cause,
            DescribeUserError::ResourceNotFound(ref cause) => cause,
            DescribeUserError::Throttling(ref cause) => cause,
            DescribeUserError::Validation(ref cause) => cause,
            DescribeUserError::Credentials(ref err) => err.description(),
            DescribeUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeUserError::ParseError(ref cause) => cause,
            DescribeUserError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeUserHierarchyGroup
#[derive(Debug, PartialEq)]
pub enum DescribeUserHierarchyGroupError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeUserHierarchyGroupError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeUserHierarchyGroupError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalServiceException" => {
                    return DescribeUserHierarchyGroupError::InternalService(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return DescribeUserHierarchyGroupError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return DescribeUserHierarchyGroupError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return DescribeUserHierarchyGroupError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return DescribeUserHierarchyGroupError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeUserHierarchyGroupError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeUserHierarchyGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeUserHierarchyGroupError {
    fn from(err: serde_json::error::Error) -> DescribeUserHierarchyGroupError {
        DescribeUserHierarchyGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeUserHierarchyGroupError {
    fn from(err: CredentialsError) -> DescribeUserHierarchyGroupError {
        DescribeUserHierarchyGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeUserHierarchyGroupError {
    fn from(err: HttpDispatchError) -> DescribeUserHierarchyGroupError {
        DescribeUserHierarchyGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeUserHierarchyGroupError {
    fn from(err: io::Error) -> DescribeUserHierarchyGroupError {
        DescribeUserHierarchyGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeUserHierarchyGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUserHierarchyGroupError {
    fn description(&self) -> &str {
        match *self {
            DescribeUserHierarchyGroupError::InternalService(ref cause) => cause,
            DescribeUserHierarchyGroupError::InvalidParameter(ref cause) => cause,
            DescribeUserHierarchyGroupError::InvalidRequest(ref cause) => cause,
            DescribeUserHierarchyGroupError::ResourceNotFound(ref cause) => cause,
            DescribeUserHierarchyGroupError::Throttling(ref cause) => cause,
            DescribeUserHierarchyGroupError::Validation(ref cause) => cause,
            DescribeUserHierarchyGroupError::Credentials(ref err) => err.description(),
            DescribeUserHierarchyGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeUserHierarchyGroupError::ParseError(ref cause) => cause,
            DescribeUserHierarchyGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeUserHierarchyStructure
#[derive(Debug, PartialEq)]
pub enum DescribeUserHierarchyStructureError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeUserHierarchyStructureError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeUserHierarchyStructureError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalServiceException" => {
                    return DescribeUserHierarchyStructureError::InternalService(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return DescribeUserHierarchyStructureError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return DescribeUserHierarchyStructureError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return DescribeUserHierarchyStructureError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return DescribeUserHierarchyStructureError::Throttling(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeUserHierarchyStructureError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return DescribeUserHierarchyStructureError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeUserHierarchyStructureError {
    fn from(err: serde_json::error::Error) -> DescribeUserHierarchyStructureError {
        DescribeUserHierarchyStructureError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeUserHierarchyStructureError {
    fn from(err: CredentialsError) -> DescribeUserHierarchyStructureError {
        DescribeUserHierarchyStructureError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeUserHierarchyStructureError {
    fn from(err: HttpDispatchError) -> DescribeUserHierarchyStructureError {
        DescribeUserHierarchyStructureError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeUserHierarchyStructureError {
    fn from(err: io::Error) -> DescribeUserHierarchyStructureError {
        DescribeUserHierarchyStructureError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeUserHierarchyStructureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUserHierarchyStructureError {
    fn description(&self) -> &str {
        match *self {
            DescribeUserHierarchyStructureError::InternalService(ref cause) => cause,
            DescribeUserHierarchyStructureError::InvalidParameter(ref cause) => cause,
            DescribeUserHierarchyStructureError::InvalidRequest(ref cause) => cause,
            DescribeUserHierarchyStructureError::ResourceNotFound(ref cause) => cause,
            DescribeUserHierarchyStructureError::Throttling(ref cause) => cause,
            DescribeUserHierarchyStructureError::Validation(ref cause) => cause,
            DescribeUserHierarchyStructureError::Credentials(ref err) => err.description(),
            DescribeUserHierarchyStructureError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeUserHierarchyStructureError::ParseError(ref cause) => cause,
            DescribeUserHierarchyStructureError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetFederationToken
#[derive(Debug, PartialEq)]
pub enum GetFederationTokenError {
    /// <p>A resource with that name already exisits.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>No user with the specified credentials was found in the Amazon Connect instance.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetFederationTokenError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetFederationTokenError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "DuplicateResourceException" => {
                    return GetFederationTokenError::DuplicateResource(String::from(error_message))
                }
                "InternalServiceException" => {
                    return GetFederationTokenError::InternalService(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return GetFederationTokenError::InvalidParameter(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return GetFederationTokenError::InvalidRequest(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return GetFederationTokenError::ResourceNotFound(String::from(error_message))
                }
                "UserNotFoundException" => {
                    return GetFederationTokenError::UserNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetFederationTokenError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetFederationTokenError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetFederationTokenError {
    fn from(err: serde_json::error::Error) -> GetFederationTokenError {
        GetFederationTokenError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetFederationTokenError {
    fn from(err: CredentialsError) -> GetFederationTokenError {
        GetFederationTokenError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetFederationTokenError {
    fn from(err: HttpDispatchError) -> GetFederationTokenError {
        GetFederationTokenError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetFederationTokenError {
    fn from(err: io::Error) -> GetFederationTokenError {
        GetFederationTokenError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetFederationTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFederationTokenError {
    fn description(&self) -> &str {
        match *self {
            GetFederationTokenError::DuplicateResource(ref cause) => cause,
            GetFederationTokenError::InternalService(ref cause) => cause,
            GetFederationTokenError::InvalidParameter(ref cause) => cause,
            GetFederationTokenError::InvalidRequest(ref cause) => cause,
            GetFederationTokenError::ResourceNotFound(ref cause) => cause,
            GetFederationTokenError::UserNotFound(ref cause) => cause,
            GetFederationTokenError::Validation(ref cause) => cause,
            GetFederationTokenError::Credentials(ref err) => err.description(),
            GetFederationTokenError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetFederationTokenError::ParseError(ref cause) => cause,
            GetFederationTokenError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListRoutingProfiles
#[derive(Debug, PartialEq)]
pub enum ListRoutingProfilesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListRoutingProfilesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListRoutingProfilesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalServiceException" => {
                    return ListRoutingProfilesError::InternalService(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return ListRoutingProfilesError::InvalidParameter(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return ListRoutingProfilesError::InvalidRequest(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListRoutingProfilesError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return ListRoutingProfilesError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return ListRoutingProfilesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListRoutingProfilesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListRoutingProfilesError {
    fn from(err: serde_json::error::Error) -> ListRoutingProfilesError {
        ListRoutingProfilesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListRoutingProfilesError {
    fn from(err: CredentialsError) -> ListRoutingProfilesError {
        ListRoutingProfilesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListRoutingProfilesError {
    fn from(err: HttpDispatchError) -> ListRoutingProfilesError {
        ListRoutingProfilesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListRoutingProfilesError {
    fn from(err: io::Error) -> ListRoutingProfilesError {
        ListRoutingProfilesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListRoutingProfilesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRoutingProfilesError {
    fn description(&self) -> &str {
        match *self {
            ListRoutingProfilesError::InternalService(ref cause) => cause,
            ListRoutingProfilesError::InvalidParameter(ref cause) => cause,
            ListRoutingProfilesError::InvalidRequest(ref cause) => cause,
            ListRoutingProfilesError::ResourceNotFound(ref cause) => cause,
            ListRoutingProfilesError::Throttling(ref cause) => cause,
            ListRoutingProfilesError::Validation(ref cause) => cause,
            ListRoutingProfilesError::Credentials(ref err) => err.description(),
            ListRoutingProfilesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListRoutingProfilesError::ParseError(ref cause) => cause,
            ListRoutingProfilesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListSecurityProfiles
#[derive(Debug, PartialEq)]
pub enum ListSecurityProfilesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListSecurityProfilesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListSecurityProfilesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalServiceException" => {
                    return ListSecurityProfilesError::InternalService(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return ListSecurityProfilesError::InvalidParameter(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return ListSecurityProfilesError::InvalidRequest(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListSecurityProfilesError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return ListSecurityProfilesError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return ListSecurityProfilesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListSecurityProfilesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListSecurityProfilesError {
    fn from(err: serde_json::error::Error) -> ListSecurityProfilesError {
        ListSecurityProfilesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListSecurityProfilesError {
    fn from(err: CredentialsError) -> ListSecurityProfilesError {
        ListSecurityProfilesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSecurityProfilesError {
    fn from(err: HttpDispatchError) -> ListSecurityProfilesError {
        ListSecurityProfilesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListSecurityProfilesError {
    fn from(err: io::Error) -> ListSecurityProfilesError {
        ListSecurityProfilesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListSecurityProfilesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSecurityProfilesError {
    fn description(&self) -> &str {
        match *self {
            ListSecurityProfilesError::InternalService(ref cause) => cause,
            ListSecurityProfilesError::InvalidParameter(ref cause) => cause,
            ListSecurityProfilesError::InvalidRequest(ref cause) => cause,
            ListSecurityProfilesError::ResourceNotFound(ref cause) => cause,
            ListSecurityProfilesError::Throttling(ref cause) => cause,
            ListSecurityProfilesError::Validation(ref cause) => cause,
            ListSecurityProfilesError::Credentials(ref err) => err.description(),
            ListSecurityProfilesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListSecurityProfilesError::ParseError(ref cause) => cause,
            ListSecurityProfilesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListUserHierarchyGroups
#[derive(Debug, PartialEq)]
pub enum ListUserHierarchyGroupsError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListUserHierarchyGroupsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListUserHierarchyGroupsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalServiceException" => {
                    return ListUserHierarchyGroupsError::InternalService(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return ListUserHierarchyGroupsError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return ListUserHierarchyGroupsError::InvalidRequest(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListUserHierarchyGroupsError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return ListUserHierarchyGroupsError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return ListUserHierarchyGroupsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListUserHierarchyGroupsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListUserHierarchyGroupsError {
    fn from(err: serde_json::error::Error) -> ListUserHierarchyGroupsError {
        ListUserHierarchyGroupsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListUserHierarchyGroupsError {
    fn from(err: CredentialsError) -> ListUserHierarchyGroupsError {
        ListUserHierarchyGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListUserHierarchyGroupsError {
    fn from(err: HttpDispatchError) -> ListUserHierarchyGroupsError {
        ListUserHierarchyGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListUserHierarchyGroupsError {
    fn from(err: io::Error) -> ListUserHierarchyGroupsError {
        ListUserHierarchyGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListUserHierarchyGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUserHierarchyGroupsError {
    fn description(&self) -> &str {
        match *self {
            ListUserHierarchyGroupsError::InternalService(ref cause) => cause,
            ListUserHierarchyGroupsError::InvalidParameter(ref cause) => cause,
            ListUserHierarchyGroupsError::InvalidRequest(ref cause) => cause,
            ListUserHierarchyGroupsError::ResourceNotFound(ref cause) => cause,
            ListUserHierarchyGroupsError::Throttling(ref cause) => cause,
            ListUserHierarchyGroupsError::Validation(ref cause) => cause,
            ListUserHierarchyGroupsError::Credentials(ref err) => err.description(),
            ListUserHierarchyGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListUserHierarchyGroupsError::ParseError(ref cause) => cause,
            ListUserHierarchyGroupsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListUsers
#[derive(Debug, PartialEq)]
pub enum ListUsersError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListUsersError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListUsersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalServiceException" => {
                    return ListUsersError::InternalService(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return ListUsersError::InvalidParameter(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return ListUsersError::InvalidRequest(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListUsersError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return ListUsersError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return ListUsersError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListUsersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListUsersError {
    fn from(err: serde_json::error::Error) -> ListUsersError {
        ListUsersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListUsersError {
    fn from(err: CredentialsError) -> ListUsersError {
        ListUsersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListUsersError {
    fn from(err: HttpDispatchError) -> ListUsersError {
        ListUsersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListUsersError {
    fn from(err: io::Error) -> ListUsersError {
        ListUsersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListUsersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUsersError {
    fn description(&self) -> &str {
        match *self {
            ListUsersError::InternalService(ref cause) => cause,
            ListUsersError::InvalidParameter(ref cause) => cause,
            ListUsersError::InvalidRequest(ref cause) => cause,
            ListUsersError::ResourceNotFound(ref cause) => cause,
            ListUsersError::Throttling(ref cause) => cause,
            ListUsersError::Validation(ref cause) => cause,
            ListUsersError::Credentials(ref err) => err.description(),
            ListUsersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListUsersError::ParseError(ref cause) => cause,
            ListUsersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartOutboundVoiceContact
#[derive(Debug, PartialEq)]
pub enum StartOutboundVoiceContactError {
    /// <p>Outbound calls to the destination number are not allowed.</p>
    DestinationNotAllowed(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The limit exceeded the maximum allowed active calls in a queue.</p>
    LimitExceeded(String),
    /// <p>The contact is not permitted.</p>
    OutboundContactNotPermitted(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StartOutboundVoiceContactError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> StartOutboundVoiceContactError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "DestinationNotAllowedException" => {
                    return StartOutboundVoiceContactError::DestinationNotAllowed(String::from(
                        error_message,
                    ))
                }
                "InternalServiceException" => {
                    return StartOutboundVoiceContactError::InternalService(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return StartOutboundVoiceContactError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return StartOutboundVoiceContactError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return StartOutboundVoiceContactError::LimitExceeded(String::from(
                        error_message,
                    ))
                }
                "OutboundContactNotPermittedException" => {
                    return StartOutboundVoiceContactError::OutboundContactNotPermitted(
                        String::from(error_message),
                    )
                }
                "ResourceNotFoundException" => {
                    return StartOutboundVoiceContactError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return StartOutboundVoiceContactError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StartOutboundVoiceContactError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartOutboundVoiceContactError {
    fn from(err: serde_json::error::Error) -> StartOutboundVoiceContactError {
        StartOutboundVoiceContactError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartOutboundVoiceContactError {
    fn from(err: CredentialsError) -> StartOutboundVoiceContactError {
        StartOutboundVoiceContactError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartOutboundVoiceContactError {
    fn from(err: HttpDispatchError) -> StartOutboundVoiceContactError {
        StartOutboundVoiceContactError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartOutboundVoiceContactError {
    fn from(err: io::Error) -> StartOutboundVoiceContactError {
        StartOutboundVoiceContactError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartOutboundVoiceContactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartOutboundVoiceContactError {
    fn description(&self) -> &str {
        match *self {
            StartOutboundVoiceContactError::DestinationNotAllowed(ref cause) => cause,
            StartOutboundVoiceContactError::InternalService(ref cause) => cause,
            StartOutboundVoiceContactError::InvalidParameter(ref cause) => cause,
            StartOutboundVoiceContactError::InvalidRequest(ref cause) => cause,
            StartOutboundVoiceContactError::LimitExceeded(ref cause) => cause,
            StartOutboundVoiceContactError::OutboundContactNotPermitted(ref cause) => cause,
            StartOutboundVoiceContactError::ResourceNotFound(ref cause) => cause,
            StartOutboundVoiceContactError::Validation(ref cause) => cause,
            StartOutboundVoiceContactError::Credentials(ref err) => err.description(),
            StartOutboundVoiceContactError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartOutboundVoiceContactError::ParseError(ref cause) => cause,
            StartOutboundVoiceContactError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StopContact
#[derive(Debug, PartialEq)]
pub enum StopContactError {
    /// <p>The contact with the specified ID is not active or does not exist.</p>
    ContactNotFound(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StopContactError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> StopContactError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ContactNotFoundException" => {
                    return StopContactError::ContactNotFound(String::from(error_message))
                }
                "InternalServiceException" => {
                    return StopContactError::InternalService(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return StopContactError::InvalidParameter(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return StopContactError::InvalidRequest(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return StopContactError::ResourceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return StopContactError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StopContactError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StopContactError {
    fn from(err: serde_json::error::Error) -> StopContactError {
        StopContactError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StopContactError {
    fn from(err: CredentialsError) -> StopContactError {
        StopContactError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopContactError {
    fn from(err: HttpDispatchError) -> StopContactError {
        StopContactError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopContactError {
    fn from(err: io::Error) -> StopContactError {
        StopContactError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopContactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopContactError {
    fn description(&self) -> &str {
        match *self {
            StopContactError::ContactNotFound(ref cause) => cause,
            StopContactError::InternalService(ref cause) => cause,
            StopContactError::InvalidParameter(ref cause) => cause,
            StopContactError::InvalidRequest(ref cause) => cause,
            StopContactError::ResourceNotFound(ref cause) => cause,
            StopContactError::Validation(ref cause) => cause,
            StopContactError::Credentials(ref err) => err.description(),
            StopContactError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopContactError::ParseError(ref cause) => cause,
            StopContactError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateUserHierarchy
#[derive(Debug, PartialEq)]
pub enum UpdateUserHierarchyError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateUserHierarchyError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateUserHierarchyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalServiceException" => {
                    return UpdateUserHierarchyError::InternalService(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return UpdateUserHierarchyError::InvalidParameter(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return UpdateUserHierarchyError::InvalidRequest(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return UpdateUserHierarchyError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return UpdateUserHierarchyError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateUserHierarchyError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateUserHierarchyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateUserHierarchyError {
    fn from(err: serde_json::error::Error) -> UpdateUserHierarchyError {
        UpdateUserHierarchyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateUserHierarchyError {
    fn from(err: CredentialsError) -> UpdateUserHierarchyError {
        UpdateUserHierarchyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateUserHierarchyError {
    fn from(err: HttpDispatchError) -> UpdateUserHierarchyError {
        UpdateUserHierarchyError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateUserHierarchyError {
    fn from(err: io::Error) -> UpdateUserHierarchyError {
        UpdateUserHierarchyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateUserHierarchyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserHierarchyError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserHierarchyError::InternalService(ref cause) => cause,
            UpdateUserHierarchyError::InvalidParameter(ref cause) => cause,
            UpdateUserHierarchyError::InvalidRequest(ref cause) => cause,
            UpdateUserHierarchyError::ResourceNotFound(ref cause) => cause,
            UpdateUserHierarchyError::Throttling(ref cause) => cause,
            UpdateUserHierarchyError::Validation(ref cause) => cause,
            UpdateUserHierarchyError::Credentials(ref err) => err.description(),
            UpdateUserHierarchyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateUserHierarchyError::ParseError(ref cause) => cause,
            UpdateUserHierarchyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateUserIdentityInfo
#[derive(Debug, PartialEq)]
pub enum UpdateUserIdentityInfoError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateUserIdentityInfoError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateUserIdentityInfoError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalServiceException" => {
                    return UpdateUserIdentityInfoError::InternalService(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return UpdateUserIdentityInfoError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return UpdateUserIdentityInfoError::InvalidRequest(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return UpdateUserIdentityInfoError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return UpdateUserIdentityInfoError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateUserIdentityInfoError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateUserIdentityInfoError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateUserIdentityInfoError {
    fn from(err: serde_json::error::Error) -> UpdateUserIdentityInfoError {
        UpdateUserIdentityInfoError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateUserIdentityInfoError {
    fn from(err: CredentialsError) -> UpdateUserIdentityInfoError {
        UpdateUserIdentityInfoError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateUserIdentityInfoError {
    fn from(err: HttpDispatchError) -> UpdateUserIdentityInfoError {
        UpdateUserIdentityInfoError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateUserIdentityInfoError {
    fn from(err: io::Error) -> UpdateUserIdentityInfoError {
        UpdateUserIdentityInfoError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateUserIdentityInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserIdentityInfoError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserIdentityInfoError::InternalService(ref cause) => cause,
            UpdateUserIdentityInfoError::InvalidParameter(ref cause) => cause,
            UpdateUserIdentityInfoError::InvalidRequest(ref cause) => cause,
            UpdateUserIdentityInfoError::ResourceNotFound(ref cause) => cause,
            UpdateUserIdentityInfoError::Throttling(ref cause) => cause,
            UpdateUserIdentityInfoError::Validation(ref cause) => cause,
            UpdateUserIdentityInfoError::Credentials(ref err) => err.description(),
            UpdateUserIdentityInfoError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateUserIdentityInfoError::ParseError(ref cause) => cause,
            UpdateUserIdentityInfoError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateUserPhoneConfig
#[derive(Debug, PartialEq)]
pub enum UpdateUserPhoneConfigError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateUserPhoneConfigError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateUserPhoneConfigError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalServiceException" => {
                    return UpdateUserPhoneConfigError::InternalService(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return UpdateUserPhoneConfigError::InvalidParameter(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return UpdateUserPhoneConfigError::InvalidRequest(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return UpdateUserPhoneConfigError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return UpdateUserPhoneConfigError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateUserPhoneConfigError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateUserPhoneConfigError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateUserPhoneConfigError {
    fn from(err: serde_json::error::Error) -> UpdateUserPhoneConfigError {
        UpdateUserPhoneConfigError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateUserPhoneConfigError {
    fn from(err: CredentialsError) -> UpdateUserPhoneConfigError {
        UpdateUserPhoneConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateUserPhoneConfigError {
    fn from(err: HttpDispatchError) -> UpdateUserPhoneConfigError {
        UpdateUserPhoneConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateUserPhoneConfigError {
    fn from(err: io::Error) -> UpdateUserPhoneConfigError {
        UpdateUserPhoneConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateUserPhoneConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserPhoneConfigError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserPhoneConfigError::InternalService(ref cause) => cause,
            UpdateUserPhoneConfigError::InvalidParameter(ref cause) => cause,
            UpdateUserPhoneConfigError::InvalidRequest(ref cause) => cause,
            UpdateUserPhoneConfigError::ResourceNotFound(ref cause) => cause,
            UpdateUserPhoneConfigError::Throttling(ref cause) => cause,
            UpdateUserPhoneConfigError::Validation(ref cause) => cause,
            UpdateUserPhoneConfigError::Credentials(ref err) => err.description(),
            UpdateUserPhoneConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateUserPhoneConfigError::ParseError(ref cause) => cause,
            UpdateUserPhoneConfigError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateUserRoutingProfile
#[derive(Debug, PartialEq)]
pub enum UpdateUserRoutingProfileError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateUserRoutingProfileError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateUserRoutingProfileError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalServiceException" => {
                    return UpdateUserRoutingProfileError::InternalService(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return UpdateUserRoutingProfileError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return UpdateUserRoutingProfileError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return UpdateUserRoutingProfileError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return UpdateUserRoutingProfileError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateUserRoutingProfileError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateUserRoutingProfileError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateUserRoutingProfileError {
    fn from(err: serde_json::error::Error) -> UpdateUserRoutingProfileError {
        UpdateUserRoutingProfileError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateUserRoutingProfileError {
    fn from(err: CredentialsError) -> UpdateUserRoutingProfileError {
        UpdateUserRoutingProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateUserRoutingProfileError {
    fn from(err: HttpDispatchError) -> UpdateUserRoutingProfileError {
        UpdateUserRoutingProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateUserRoutingProfileError {
    fn from(err: io::Error) -> UpdateUserRoutingProfileError {
        UpdateUserRoutingProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateUserRoutingProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserRoutingProfileError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserRoutingProfileError::InternalService(ref cause) => cause,
            UpdateUserRoutingProfileError::InvalidParameter(ref cause) => cause,
            UpdateUserRoutingProfileError::InvalidRequest(ref cause) => cause,
            UpdateUserRoutingProfileError::ResourceNotFound(ref cause) => cause,
            UpdateUserRoutingProfileError::Throttling(ref cause) => cause,
            UpdateUserRoutingProfileError::Validation(ref cause) => cause,
            UpdateUserRoutingProfileError::Credentials(ref err) => err.description(),
            UpdateUserRoutingProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateUserRoutingProfileError::ParseError(ref cause) => cause,
            UpdateUserRoutingProfileError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateUserSecurityProfiles
#[derive(Debug, PartialEq)]
pub enum UpdateUserSecurityProfilesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateUserSecurityProfilesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateUserSecurityProfilesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalServiceException" => {
                    return UpdateUserSecurityProfilesError::InternalService(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return UpdateUserSecurityProfilesError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return UpdateUserSecurityProfilesError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return UpdateUserSecurityProfilesError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return UpdateUserSecurityProfilesError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateUserSecurityProfilesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateUserSecurityProfilesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateUserSecurityProfilesError {
    fn from(err: serde_json::error::Error) -> UpdateUserSecurityProfilesError {
        UpdateUserSecurityProfilesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateUserSecurityProfilesError {
    fn from(err: CredentialsError) -> UpdateUserSecurityProfilesError {
        UpdateUserSecurityProfilesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateUserSecurityProfilesError {
    fn from(err: HttpDispatchError) -> UpdateUserSecurityProfilesError {
        UpdateUserSecurityProfilesError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateUserSecurityProfilesError {
    fn from(err: io::Error) -> UpdateUserSecurityProfilesError {
        UpdateUserSecurityProfilesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateUserSecurityProfilesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserSecurityProfilesError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserSecurityProfilesError::InternalService(ref cause) => cause,
            UpdateUserSecurityProfilesError::InvalidParameter(ref cause) => cause,
            UpdateUserSecurityProfilesError::InvalidRequest(ref cause) => cause,
            UpdateUserSecurityProfilesError::ResourceNotFound(ref cause) => cause,
            UpdateUserSecurityProfilesError::Throttling(ref cause) => cause,
            UpdateUserSecurityProfilesError::Validation(ref cause) => cause,
            UpdateUserSecurityProfilesError::Credentials(ref err) => err.description(),
            UpdateUserSecurityProfilesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateUserSecurityProfilesError::ParseError(ref cause) => cause,
            UpdateUserSecurityProfilesError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Amazon Connect API. Amazon Connect clients implement this trait.
pub trait Connect {
    /// <p>Creates a new user account in your Amazon Connect instance.</p>
    fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> RusotoFuture<CreateUserResponse, CreateUserError>;

    /// <p>Deletes a user account from Amazon Connect.</p>
    fn delete_user(&self, input: DeleteUserRequest) -> RusotoFuture<(), DeleteUserError>;

    /// <p>Returns a <code>User</code> object that contains information about the user account specified by the <code>UserId</code>.</p>
    fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> RusotoFuture<DescribeUserResponse, DescribeUserError>;

    /// <p>Returns a <code>HierarchyGroup</code> object that includes information about a hierarchy group in your instance.</p>
    fn describe_user_hierarchy_group(
        &self,
        input: DescribeUserHierarchyGroupRequest,
    ) -> RusotoFuture<DescribeUserHierarchyGroupResponse, DescribeUserHierarchyGroupError>;

    /// <p>Returns a <code>HiearchyGroupStructure</code> object, which contains data about the levels in the agent hierarchy.</p>
    fn describe_user_hierarchy_structure(
        &self,
        input: DescribeUserHierarchyStructureRequest,
    ) -> RusotoFuture<DescribeUserHierarchyStructureResponse, DescribeUserHierarchyStructureError>;

    /// <p>Retrieves a token for federation.</p>
    fn get_federation_token(
        &self,
        input: GetFederationTokenRequest,
    ) -> RusotoFuture<GetFederationTokenResponse, GetFederationTokenError>;

    /// <p>Returns an array of <code>RoutingProfileSummary</code> objects that includes information about the routing profiles in your instance.</p>
    fn list_routing_profiles(
        &self,
        input: ListRoutingProfilesRequest,
    ) -> RusotoFuture<ListRoutingProfilesResponse, ListRoutingProfilesError>;

    /// <p>Returns an array of SecurityProfileSummary objects that contain information about the security profiles in your instance, including the ARN, Id, and Name of the security profile.</p>
    fn list_security_profiles(
        &self,
        input: ListSecurityProfilesRequest,
    ) -> RusotoFuture<ListSecurityProfilesResponse, ListSecurityProfilesError>;

    /// <p>Returns a <code>UserHierarchyGroupSummaryList</code>, which is an array of <code>HierarchyGroupSummary</code> objects that contain information about the hierarchy groups in your instance.</p>
    fn list_user_hierarchy_groups(
        &self,
        input: ListUserHierarchyGroupsRequest,
    ) -> RusotoFuture<ListUserHierarchyGroupsResponse, ListUserHierarchyGroupsError>;

    /// <p>Returns a <code>UserSummaryList</code>, which is an array of <code>UserSummary</code> objects.</p>
    fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> RusotoFuture<ListUsersResponse, ListUsersError>;

    /// <p>The <code>StartOutboundVoiceContact</code> operation initiates a contact flow to place an outbound call to a customer.</p> <p>There is a throttling limit placed on usage of the API that includes a RateLimit of 2 per second, and a BurstLimit of 5 per second.</p> <p>If you are using an IAM account, it must have permission to the <code>connect:StartOutboundVoiceContact</code> action.</p>
    fn start_outbound_voice_contact(
        &self,
        input: StartOutboundVoiceContactRequest,
    ) -> RusotoFuture<StartOutboundVoiceContactResponse, StartOutboundVoiceContactError>;

    /// <p>Ends the contact initiated by the <code>StartOutboundVoiceContact</code> operation.</p> <p>If you are using an IAM account, it must have permission to the <code>connect:StopContact</code> action.</p>
    fn stop_contact(
        &self,
        input: StopContactRequest,
    ) -> RusotoFuture<StopContactResponse, StopContactError>;

    /// <p>Assigns the specified hierarchy group to the user.</p>
    fn update_user_hierarchy(
        &self,
        input: UpdateUserHierarchyRequest,
    ) -> RusotoFuture<(), UpdateUserHierarchyError>;

    /// <p>Updates the identity information for the specified user in a <code>UserIdentityInfo</code> object, including email, first name, and last name.</p>
    fn update_user_identity_info(
        &self,
        input: UpdateUserIdentityInfoRequest,
    ) -> RusotoFuture<(), UpdateUserIdentityInfoError>;

    /// <p>Updates the phone configuration settings in the <code>UserPhoneConfig</code> object for the specified user.</p>
    fn update_user_phone_config(
        &self,
        input: UpdateUserPhoneConfigRequest,
    ) -> RusotoFuture<(), UpdateUserPhoneConfigError>;

    /// <p>Assigns the specified routing profile to a user.</p>
    fn update_user_routing_profile(
        &self,
        input: UpdateUserRoutingProfileRequest,
    ) -> RusotoFuture<(), UpdateUserRoutingProfileError>;

    /// <p>Update the security profiles assigned to the user.</p>
    fn update_user_security_profiles(
        &self,
        input: UpdateUserSecurityProfilesRequest,
    ) -> RusotoFuture<(), UpdateUserSecurityProfilesError>;
}
/// A client for the Amazon Connect API.
pub struct ConnectClient {
    client: Client,
    region: region::Region,
}

impl ConnectClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ConnectClient {
        ConnectClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ConnectClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        ConnectClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Connect for ConnectClient {
    /// <p>Creates a new user account in your Amazon Connect instance.</p>
    fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> RusotoFuture<CreateUserResponse, CreateUserError> {
        let request_uri = format!("/users/{instance_id}", instance_id = input.instance_id);

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateUserResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a user account from Amazon Connect.</p>
    fn delete_user(&self, input: DeleteUserRequest) -> RusotoFuture<(), DeleteUserError> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("DELETE", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a <code>User</code> object that contains information about the user account specified by the <code>UserId</code>.</p>
    fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> RusotoFuture<DescribeUserResponse, DescribeUserError> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeUserResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a <code>HierarchyGroup</code> object that includes information about a hierarchy group in your instance.</p>
    fn describe_user_hierarchy_group(
        &self,
        input: DescribeUserHierarchyGroupRequest,
    ) -> RusotoFuture<DescribeUserHierarchyGroupResponse, DescribeUserHierarchyGroupError> {
        let request_uri = format!(
            "/user-hierarchy-groups/{instance_id}/{hierarchy_group_id}",
            hierarchy_group_id = input.hierarchy_group_id,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeUserHierarchyGroupResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeUserHierarchyGroupError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a <code>HiearchyGroupStructure</code> object, which contains data about the levels in the agent hierarchy.</p>
    fn describe_user_hierarchy_structure(
        &self,
        input: DescribeUserHierarchyStructureRequest,
    ) -> RusotoFuture<DescribeUserHierarchyStructureResponse, DescribeUserHierarchyStructureError>
    {
        let request_uri = format!(
            "/user-hierarchy-structure/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeUserHierarchyStructureResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeUserHierarchyStructureError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves a token for federation.</p>
    fn get_federation_token(
        &self,
        input: GetFederationTokenRequest,
    ) -> RusotoFuture<GetFederationTokenResponse, GetFederationTokenError> {
        let request_uri = format!(
            "/user/federate/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetFederationTokenResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetFederationTokenError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an array of <code>RoutingProfileSummary</code> objects that includes information about the routing profiles in your instance.</p>
    fn list_routing_profiles(
        &self,
        input: ListRoutingProfilesRequest,
    ) -> RusotoFuture<ListRoutingProfilesResponse, ListRoutingProfilesError> {
        let request_uri = format!(
            "/routing-profiles-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListRoutingProfilesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListRoutingProfilesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns an array of SecurityProfileSummary objects that contain information about the security profiles in your instance, including the ARN, Id, and Name of the security profile.</p>
    fn list_security_profiles(
        &self,
        input: ListSecurityProfilesRequest,
    ) -> RusotoFuture<ListSecurityProfilesResponse, ListSecurityProfilesError> {
        let request_uri = format!(
            "/security-profiles-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListSecurityProfilesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListSecurityProfilesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a <code>UserHierarchyGroupSummaryList</code>, which is an array of <code>HierarchyGroupSummary</code> objects that contain information about the hierarchy groups in your instance.</p>
    fn list_user_hierarchy_groups(
        &self,
        input: ListUserHierarchyGroupsRequest,
    ) -> RusotoFuture<ListUserHierarchyGroupsResponse, ListUserHierarchyGroupsError> {
        let request_uri = format!(
            "/user-hierarchy-groups-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListUserHierarchyGroupsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListUserHierarchyGroupsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a <code>UserSummaryList</code>, which is an array of <code>UserSummary</code> objects.</p>
    fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> RusotoFuture<ListUsersResponse, ListUsersError> {
        let request_uri = format!(
            "/users-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListUsersResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListUsersError::from_response(response))),
                )
            }
        })
    }

    /// <p>The <code>StartOutboundVoiceContact</code> operation initiates a contact flow to place an outbound call to a customer.</p> <p>There is a throttling limit placed on usage of the API that includes a RateLimit of 2 per second, and a BurstLimit of 5 per second.</p> <p>If you are using an IAM account, it must have permission to the <code>connect:StartOutboundVoiceContact</code> action.</p>
    fn start_outbound_voice_contact(
        &self,
        input: StartOutboundVoiceContactRequest,
    ) -> RusotoFuture<StartOutboundVoiceContactResponse, StartOutboundVoiceContactError> {
        let request_uri = "/contact/outbound-voice";

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<StartOutboundVoiceContactResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartOutboundVoiceContactError::from_response(response))
                }))
            }
        })
    }

    /// <p>Ends the contact initiated by the <code>StartOutboundVoiceContact</code> operation.</p> <p>If you are using an IAM account, it must have permission to the <code>connect:StopContact</code> action.</p>
    fn stop_contact(
        &self,
        input: StopContactRequest,
    ) -> RusotoFuture<StopContactResponse, StopContactError> {
        let request_uri = "/contact/stop";

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<StopContactResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopContactError::from_response(response))),
                )
            }
        })
    }

    /// <p>Assigns the specified hierarchy group to the user.</p>
    fn update_user_hierarchy(
        &self,
        input: UpdateUserHierarchyRequest,
    ) -> RusotoFuture<(), UpdateUserHierarchyError> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/hierarchy",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateUserHierarchyError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates the identity information for the specified user in a <code>UserIdentityInfo</code> object, including email, first name, and last name.</p>
    fn update_user_identity_info(
        &self,
        input: UpdateUserIdentityInfoRequest,
    ) -> RusotoFuture<(), UpdateUserIdentityInfoError> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/identity-info",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateUserIdentityInfoError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates the phone configuration settings in the <code>UserPhoneConfig</code> object for the specified user.</p>
    fn update_user_phone_config(
        &self,
        input: UpdateUserPhoneConfigRequest,
    ) -> RusotoFuture<(), UpdateUserPhoneConfigError> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/phone-config",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateUserPhoneConfigError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Assigns the specified routing profile to a user.</p>
    fn update_user_routing_profile(
        &self,
        input: UpdateUserRoutingProfileRequest,
    ) -> RusotoFuture<(), UpdateUserRoutingProfileError> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/routing-profile",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateUserRoutingProfileError::from_response(response))
                }))
            }
        })
    }

    /// <p>Update the security profiles assigned to the user.</p>
    fn update_user_security_profiles(
        &self,
        input: UpdateUserSecurityProfilesRequest,
    ) -> RusotoFuture<(), UpdateUserSecurityProfilesError> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/security-profiles",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateUserSecurityProfilesError::from_response(response))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
