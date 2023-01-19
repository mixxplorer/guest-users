#![allow(clippy::extra_unused_lifetimes)]

extern crate diesel;

use crate::db::schema::groups;
use crate::db::schema::user_group_memberships;
use crate::db::schema::users;

#[derive(Identifiable, Insertable, AsChangeset, Queryable, Debug, Clone)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub user_group_id: i32,
    pub home_path: String,
}

#[derive(Identifiable, Insertable, AsChangeset, Queryable, Debug, Clone)]
#[table_name = "groups"]
pub struct Group {
    pub id: i32,
    pub group_name: String,
}

#[derive(Identifiable, Insertable, AsChangeset, Associations, Queryable, Debug, Clone)]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Group, foreign_key = "group_id")]
#[table_name = "user_group_memberships"]
pub struct UserGroupMembership {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32,
}
