//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "asset"
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub idx: i64,
    pub asset_id: String,
    pub ticker: String,
    pub name: String,
    pub precision: u8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Idx,
    AssetId,
    Ticker,
    Name,
    Precision,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Idx,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AssetTransfer,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Idx => ColumnType::BigInteger.def(),
            Self::AssetId => ColumnType::String(None).def().unique(),
            Self::Ticker => ColumnType::String(None).def(),
            Self::Name => ColumnType::String(None).def(),
            Self::Precision => ColumnType::SmallInteger.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AssetTransfer => Entity::has_many(super::asset_transfer::Entity).into(),
        }
    }
}

impl Related<super::asset_transfer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AssetTransfer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
