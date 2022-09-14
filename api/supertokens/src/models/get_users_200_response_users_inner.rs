/*
 * Core Driver Interface
 *
 * This is the API exposed by the SuperTokens Core. To be consumed by your backend only.
 *
 * The version of the OpenAPI document: 2.15.1
 * Contact: team@supertokens.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUsers200ResponseUsersInner {
    #[serde(rename = "recipeId")]
    pub recipe_id: RecipeId,
    #[serde(rename = "user")]
    pub user: Box<crate::models::GetUsers200ResponseUsersInnerUser>,
}

impl GetUsers200ResponseUsersInner {
    pub fn new(recipe_id: RecipeId, user: crate::models::GetUsers200ResponseUsersInnerUser) -> GetUsers200ResponseUsersInner {
        GetUsers200ResponseUsersInner {
            recipe_id,
            user: Box::new(user),
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecipeId {
    #[serde(rename = "emailpassword")]
    Emailpassword,
    #[serde(rename = "thirdparty")]
    Thirdparty,
    #[serde(rename = "passwordless")]
    Passwordless,
}

impl Default for RecipeId {
    fn default() -> RecipeId {
        Self::Emailpassword
    }
}
