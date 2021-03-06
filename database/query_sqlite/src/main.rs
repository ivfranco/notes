use query_sqlite::Value;
use rusqlite::{params, Connection, Result, Row};

fn main() -> Result<()> {
    exercise_5_2_1()
}

fn collect(row: &Row) -> Result<Vec<Value>> {
    let mut vec = vec![];
    for i in 0..row.column_count() {
        vec.push(row.get(i)?);
    }
    Ok(vec)
}

fn exec(conn: &Connection, stmt: &str) -> Result<()> {
    println!("{}", stmt);
    let mut stmt = conn.prepare(stmt)?;
    let rows = stmt.query_map(params!(), collect)?;
    for row in rows {
        println!("{:?}", row?);
    }
    Ok(())
}

fn exercise_5_2_1() -> Result<()> {
    const R: [(i32, i32); 5] = [(0, 1), (2, 3), (0, 1), (2, 4), (3, 4)];
    const S: [(i32, i32); 6] = [(0, 1), (2, 4), (2, 5), (3, 4), (0, 2), (3, 4)];

    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE R (
            A   INT,
            B   INT
        );",
        params!(),
    )?;
    conn.execute(
        "CREATE TABLE S (
            B   INT,
            C   INT
        );",
        params!(),
    )?;

    for (a, b) in &R {
        conn.execute("INSERT INTO R (A, B) VALUES (?1, ?2)", &[a, b])?;
    }
    for (b, c) in &S {
        conn.execute("INSERT INTO S (B, C) VALUES (?1, ?2)", &[b, c])?;
    }

    let exec = |stmt: &str| exec(&conn, stmt);

    exec("SELECT A + B, A * A, B * B FROM R;")?;
    exec("SELECT B + 1, C - 1 FROM S;")?;
    exec("SELECT * FROM R ORDER BY B, A;")?;
    exec("SELECT * FROM S ORDER BY B, C;")?;
    exec("SELECT DISTINCT * FROM R;")?;
    exec("SELECT DISTINCT * FROM S;")?;
    exec("SELECT A, SUM(B) FROM R GROUP BY A;")?;
    exec("SELECT B, AVG(C) FROM S GROUP BY B;")?;
    exec("SELECT A FROM R GROUP BY A")?;
    exec("SELECT A, MAX(C) FROM (R INNER JOIN S ON R.B = S.B) GROUP BY A;")?;
    exec("SELECT A, R.B, C FROM (R LEFT OUTER JOIN S ON R.B = S.B);")?;
    exec("SELECT A, S.B, C FROM (S LEFT OUTER JOIN R ON R.B = S.B);")?;
    exec(
        "SELECT A, R.B, C FROM (R LEFT OUTER JOIN S ON R.B = S.B)
UNION ALL
SELECT A, S.B, C FROM (S LEFT OUTER JOIN R ON R.B = S.B) WHERE A IS NULL;",
    )?;
    exec(
        "SELECT A, R.B, S.B, C FROM (R LEFT OUTER JOIN S ON R.B < S.B)
UNION ALL
SELECT A, R.B, S.B, C FROM (S LEFT OUTER JOIN R ON R.B < S.B) WHERE A IS NULL;",
    )?;

    Ok(())
}
