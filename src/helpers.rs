// use std::fmt::Write;

// use diesel::deserialize::{self, FromSql, FromSqlRow};
// use diesel::expression::AsExpression;
// use diesel::pg::Pg;
// use diesel::serialize::{self, ToSql};
// use diesel::sql_types::{Decimal, Unsigned};

// #[derive(AsExpression, FromSqlRow)]
// #[sql_type = "Unsigned<Decimal>"]
// struct DecimalWrapper(Decimal);

// impl FromSql<Unsigned<Decimal>, Pg> for DecimalWrapper {
//     fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
//         <Decimal as FromSql<Decimal, Pg>>::from_sql(bytes).map(Self)
//     }
// }

// impl ToSql<Unsigned<Decimal>, Pg> for DecimalWrapper {
//     fn to_sql<W: Write>(&self, out: &mut serialize::Output<'_, W, DB>) -> serialize::Result {
//         <_ as ToSql<Decimal, Mysql>>::to_sql(&self.0, out)
//     }
// }
