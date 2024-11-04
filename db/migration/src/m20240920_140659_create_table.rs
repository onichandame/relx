use extension::postgres::Type;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        if matches!(
            db.get_database_backend(),
            sea_orm::DatabaseBackend::Postgres
        ) {
            manager
                .create_type(
                    Type::create()
                        .as_enum(SetType::Enum)
                        .values([SetType::Direct, SetType::ObjectRelation])
                        .to_owned(),
                )
                .await?;
            manager
                .create_type(
                    Type::create()
                        .as_enum(SetOperationType::Enum)
                        .values([
                            SetOperationType::Union,
                            SetOperationType::Intersection,
                            SetOperationType::Difference,
                        ])
                        .to_owned(),
                )
                .await?;
        }

        manager
            .create_table(
                Table::create()
                    .table(Set::Table)
                    .if_not_exists()
                    .col(pk_auto(Set::Id))
                    .col(ColumnDef::new(Set::Type).custom(SetType::Enum).not_null())
                    .col(string_null(Set::Namespace))
                    .col(string_null(Set::Name))
                    .col(string_null(Set::Relation))
                    .col(integer_null(Set::ObjectId))
                    .col(boolean_null(Set::IsFinite))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_set_object_id")
                            .from(Set::Table, Set::ObjectId)
                            .to(Set::Table, Set::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("idx_type_namespace_name")
                    .table(Set::Table)
                    .col(Set::Type)
                    .col(Set::Namespace)
                    .col(Set::Name)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("idx_type_relation_object_id")
                    .table(Set::Table)
                    .col(Set::Type)
                    .col(Set::Relation)
                    .col(Set::ObjectId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SetOperation::Table)
                    .if_not_exists()
                    .col(pk_auto(SetOperation::Id))
                    .col(integer(SetOperation::LeftOperandId).not_null())
                    .col(integer(SetOperation::RightOperandId).not_null())
                    .col(
                        ColumnDef::new(SetOperation::Type)
                            .custom(SetOperationType::Enum)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_set_operation_left_operand_id")
                            .from(SetOperation::Table, SetOperation::LeftOperandId)
                            .to(Set::Table, Set::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_set_operation_right_operand_id")
                            .from(SetOperation::Table, SetOperation::RightOperandId)
                            .to(Set::Table, Set::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("idx_left_operand_id_right_operand_id")
                    .table(SetOperation::Table)
                    .col(SetOperation::LeftOperandId)
                    .col(SetOperation::RightOperandId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Set::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SetOperation::Table).to_owned())
            .await?;

        if matches!(
            manager.get_connection().get_database_backend(),
            sea_orm::DatabaseBackend::Postgres
        ) {
            manager
                .drop_type(Type::drop().name(SetType::Enum).to_owned())
                .await?;
            manager
                .drop_type(Type::drop().name(SetOperationType::Enum).to_owned())
                .await?;
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Set {
    Table,
    Id,
    Type,
    Namespace,
    Name,
    Relation,
    ObjectId,
    /// Whether the Set is finite or not. Only finite sets can be fully expanded
    ///
    /// The sets that are not finite are:
    /// - infinity set: `infinite:*`
    /// - (any set) U (infinite set)
    /// - (infinite set) \ (any set)
    IsFinite,
}

#[derive(DeriveIden)]
enum SetType {
    #[sea_orm(iden = "set_type")]
    Enum,
    /// A single entity
    ///
    /// fields:
    /// - Namespace
    /// - Name
    ///   - `this`: points to the current entity in the subquestion
    ///   - `*`: points to all entities in the same namespace
    ///   - other: represents a specific entity
    Direct,
    /// A set of entities which have the specified relation to the specified object
    ///
    /// fields:
    /// - Relation
    /// - ObjectId: The Id of the Set defining the object in the subquestion
    ObjectRelation,
}

/// See <https://en.wikipedia.org/wiki/Set_(mathematics)#Basic_operations>
///
/// An operation on two sets produces a final set.
#[derive(DeriveIden)]
enum SetOperation {
    Table,
    Id,
    LeftOperandId,
    RightOperandId,
    Type,
}

#[derive(DeriveIden)]
enum SetOperationType {
    #[sea_orm(iden = "type")]
    Enum,
    /// Left ∪ Right
    Union,
    /// Left ∩ Right
    Intersection,
    /// Left \ Right
    Difference,
}
