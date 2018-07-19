initSidebarItems({"struct":[["SqliteConnection","Connections for the SQLite backend. Unlike other backends, \"connection URLs\" for SQLite are file paths or special identifiers like `:memory`."]],"trait":[["BelongingToDsl","Constructs a query that finds record(s) based on directional association with other record(s)."],["GroupedBy","The `grouped_by` function groups records by their parent."],["Insertable","Represents that a structure can be used to insert a new row into the database. This is automatically implemented for `&[T]` and `&Vec<T>` for inserting more than one record."],["JoinOnDsl","Specify the `ON` clause for a join statement. This will override any implicit `ON` clause that would come from [`joinable!`]"],["JoinTo","Indicates that two tables can be joined without an explicit `ON` clause."],["SaveChangesDsl","Sugar for types which implement both `AsChangeset` and `Identifiable`"]]});