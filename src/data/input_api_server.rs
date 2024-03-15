//! Функции для работы с АПИ-сервером
use std::collections::HashMap;

use api_tools::client::{api_query::*, api_request::ApiRequest};

use crate::{data::structs::*, error::Error};
/*
/// Создание тестовой БД
#[allow(dead_code)]
pub fn create_test_db(db_name: &str) -> Result<(), Error> {
    //   let script = include_str!("../../src/data/sql/tables/create_postgres_db.sql");

    let mut request = ApiRequest::new(
        "parent",
        "0.0.0.0:8080",
        "auth_token",
        ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(db_name, "")), false),
        false,
        false,
    );

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/ship.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/center_waterline.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/rad_long.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/mean_draught.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/center_draught.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/frame.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/frame_area.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/load_space.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/load_constant.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/tank.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/tank_center.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/tank_inertia.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);
    Ok(())
}
*/
/// Чтение данных из БД. Функция читает данные за несколько запросов,
/// парсит их и проверяет данные на корректность.
pub fn get_data(db_name: &str, ship_id: usize) -> Result<ParsedShipData, Error> {
    let mut request = ApiRequest::new(
        "parent",
        "0.0.0.0:8080",
        "auth_token",
        ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(db_name, "")), false),
        false,
        false,
    );
    log::info!("input_api_server read begin");
    let ship = ShipArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!("SELECT key, value FROM ship WHERE ship_id={};", ship_id),
    )?)?;
    //dbg!(&ship);
    log::info!("input_api_server ship read ok");
    let center_waterline = CenterWaterlineArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT key, value FROM center_waterline WHERE ship_id={};",
            ship_id
        ),
    )?)?;
    //    dbg!(&center_waterline);
    log::info!("input_api_server center_waterline read ok");
    let center_draught_shift = CenterDraughtShiftDataArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT key, value_x, value_y, value_z FROM center_draught WHERE ship_id={};",
            ship_id
        ),
    )?)?;
    //  dbg!(&center_draught_shift);
    log::info!("input_api_server center_draught_shift read ok");
    let mean_draught = MeanDraughtDataArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT key, value FROM mean_draught WHERE ship_id={};",
            ship_id
        ),
    )?)?;
    //    dbg!(&mean_draught);
    log::info!("input_api_server mean_draught read ok");
    let rad_long = RadLongDataArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!("SELECT key, value FROM rad_long WHERE ship_id={};", ship_id),
    )?)?;
    //    dbg!(&rad_long);
    log::info!("input_api_server rad_long read ok");
    let rad_lat = RadLongDataArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!("SELECT key, value FROM rad_lat WHERE ship_id={};", ship_id),
    )?)?;
    //    dbg!(&rad_lat);
    log::info!("input_api_server rad_lat read ok");
    let frame = FrameDataArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT index, key, value FROM frame WHERE ship_id={};",
            ship_id
        ),
    )?)?;
    //    dbg!(&frame);
    log::info!("input_api_server frame read ok");
    let frame_area = FrameAreaData::new(
        FrameAreaArray::parse(&fetch_query(
            &mut request,
            db_name,
            format!(
                "SELECT frame_index, key, value FROM frame_area WHERE ship_id={};",
                ship_id
            ),
        )?)?
        .data(),
    );
    //    dbg!(&frame_area);
    log::info!("input_api_server frame_area read ok");
    let load_space = LoadSpaceArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT space_id, key, value FROM load_space WHERE ship_id={};",
            ship_id
        ),
    )?)?;
    //    dbg!(&load_space);
    log::info!("input_api_server load_space read ok");
    let load_constant = LoadConstantArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT frame_space_index, key, value FROM load_constant WHERE ship_id={};",
            ship_id
        ),
    )?)?;
    //    dbg!(&load_constant);
    log::info!("input_api_server load_constant read ok");
    let tank = TankDataArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT tank_id, key, value FROM tank WHERE ship_id={};",
            ship_id
        ),
    )?)?;
    // dbg!(&tank);
    log::info!("input_api_server tank read ok");
    let tank_center = CenterVolumeData::new(
        CenterVolumeArray::parse(&fetch_query(
            &mut request,
            db_name,
            format!(
                "SELECT tank_id, key, value_x, value_y, value_z FROM tank_center WHERE tank_id={};",
                ship_id
            ),
        )?)?
        .data(),
    );
    // dbg!(&tank_center);
    log::info!("input_api_server tank_center read ok");
    let tank_inertia = FreeMomentInertiaData::new(
        FreeMomentInertiaArray::parse(&fetch_query(
            &mut request,
            db_name,
            format!(
                "SELECT tank_id, key, value_x, value_y FROM tank_center WHERE tank_id={};",
                ship_id
            ),
        )?)?
        .data(),
    );
    //  dbg!(&tank_inertia);
    log::info!("input_api_server tank_inertia read ok");
    log::info!("input_api_server read ok");
    ParsedShipData::parse(
        ship_id,
        ship,
        center_waterline,
        rad_long,
        rad_lat,
        mean_draught,
        center_draught_shift,
        frame,
        frame_area,
        load_constant,
        load_space,
        tank,
        tank_center,
        tank_inertia,
    )
}
/// Вспомогательная функция для выполнения запроса к апи-серверу
fn fetch_query(
    request: &mut ApiRequest,
    database: impl Into<String>,
    sql: impl Into<String>,
) -> Result<Vec<u8>, Error> {
    let query = ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(database, sql)), false);
    Ok(request.fetch(&query, true)?)
}

/// Чтение данных из БД. Функция читает данные за несколько запросов,
/// парсит их и проверяет данные на корректность.
pub async fn async_get_data(db_name: &str, ship_id: usize) -> Result<ParsedShipData, Error> {
    log::info!("input_api_server read begin");
    let ship = async_query(
        db_name,
        format!("SELECT key, value FROM ship WHERE ship_id={};", ship_id),
    );
    //dbg!(&ship);
    log::info!("input_api_server ship read ok");
    let center_waterline = async_query(
        db_name,
        format!(
            "SELECT key, value FROM center_waterline WHERE ship_id={};",
            ship_id
        ),
    );
    //    dbg!(&center_waterline);
    log::info!("input_api_server center_waterline read ok");
    let center_draught_shift = async_query(
        db_name,
        format!(
            "SELECT key, value_x, value_y, value_z FROM center_draught WHERE ship_id={};",
            ship_id
        ),
    );
    //  dbg!(&center_draught_shift);
    log::info!("input_api_server center_draught_shift read ok");
    let mean_draught = async_query(
        db_name,
        format!(
            "SELECT key, value FROM mean_draught WHERE ship_id={};",
            ship_id
        ),
    );
    //    dbg!(&mean_draught);
    log::info!("input_api_server mean_draught read ok");
    let rad_long = async_query(
        db_name,
        format!("SELECT key, value FROM rad_long WHERE ship_id={};", ship_id),
    );
    //    dbg!(&rad_long);
    log::info!("input_api_server rad_long read ok");
    let rad_lat = async_query(
        db_name,
        format!("SELECT key, value FROM rad_lat WHERE ship_id={};", ship_id),
    );
    //    dbg!(&rad_lat);
    log::info!("input_api_server rad_lat read ok");
    let frame = async_query(
        db_name,
        format!(
            "SELECT index, key, value FROM frame WHERE ship_id={};",
            ship_id
        ),
    );
    //    dbg!(&frame);
    log::info!("input_api_server frame read ok");
    let frame_area = async_query(
        db_name,
        format!(
            "SELECT frame_index, key, value FROM frame_area WHERE ship_id={};",
            ship_id
        ),
    );
    //    dbg!(&frame_area);
    log::info!("input_api_server frame_area read ok");
    let load_space = async_query(
        db_name,
        format!(
            "SELECT space_id, key, value FROM load_space WHERE ship_id={};",
            ship_id
        ),
    );
    //    dbg!(&load_space);
    log::info!("input_api_server load_space read ok");
    let load_constant = async_query(
        db_name,
        format!(
            "SELECT frame_space_index, key, value FROM load_constant WHERE ship_id={};",
            ship_id
        ),
    );
    //    dbg!(&load_constant);
    log::info!("input_api_server load_constant read ok");
    let tank = async_query(
        db_name,
        format!(
            "SELECT tank_id, key, value FROM tank WHERE ship_id={};",
            ship_id
        ),
    );
    // dbg!(&tank);
    log::info!("input_api_server tank read ok");
    let tank_center = async_query(
        db_name,
        format!(
            "SELECT tank_id, key, value_x, value_y, value_z FROM tank_center WHERE tank_id={};",
            ship_id
        ),
    );
    // dbg!(&tank_center);
    log::info!("input_api_server tank_center read ok");
    let tank_inertia = async_query(
        db_name,
        format!(
            "SELECT tank_id, key, value_x, value_y FROM tank_center WHERE tank_id={};",
            ship_id
        ),
    );

    //  dbg!(&tank_inertia);
    log::info!("input_api_server tank_inertia read ok");

    let (
        ship,
        center_waterline,
        center_draught_shift,
        mean_draught,
        rad_long,
        rad_lat,
        frame,
        frame_area,
        load_space,
        load_constant,
        tank,
        tank_center,
        tank_inertia,
    ) = futures::join!(
        ship,
        center_waterline,
        center_draught_shift,
        mean_draught,
        rad_long,
        rad_lat,
        frame,
        frame_area,
        load_space,
        load_constant,
        tank,
        tank_center,
        tank_inertia,
    );

    log::info!("input_api_server read ok");
    ParsedShipData::parse(
        ship_id,
        ShipArray::parse(&ship?)?,
        CenterWaterlineArray::parse(&center_waterline?)?,
        RadLongDataArray::parse(&rad_long?)?,
        RadLatDataArray::parse(&rad_lat?)?,
        MeanDraughtDataArray::parse(&mean_draught?)?,
        CenterDraughtShiftDataArray::parse(&center_draught_shift?)?,
        FrameDataArray::parse(&frame?)?,
        FrameAreaData::new(
            FrameAreaArray::parse(&frame_area?)?
            .data()),
            LoadConstantArray::parse(&load_constant?)?,
        LoadSpaceArray::parse(&load_space?)?,
        TankDataArray::parse(&tank?)?,
        CenterVolumeData::new(
            CenterVolumeArray::parse(&tank_center?)?
            .data()),
            FreeMomentInertiaData::new(
                FreeMomentInertiaArray::parse(&tank_inertia?)?.data()),
    )
}

/// Вспомогательная функция для выполнения запроса к апи-серверу
async fn async_query(database: impl Into<String>, sql: String) -> Result<Vec<u8>, Error> {
    let query = ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(database, sql.clone())), false);
    let mut request = ApiRequest::new(
        "parent",
        "0.0.0.0:8080",
        "auth_token",
        query.clone(),
        false,
        false,
    );
    Ok(request.fetch(&query, false)?)
}

/*
/// Чтение данных из БД. Функция читает данные за несколько запросов,
/// парсит их и проверяет данные на корректность.
pub async fn async_get_data(db_name: &str, ship_id: usize) -> Result<ParsedShipData, Error> {
    log::info!("input_api_server read begin");
    let ship = async {
        println!("1");
        ShipArray::parse(&async_query(
            db_name,
            format!("SELECT key, value FROM ship WHERE ship_id={};", ship_id),
        )?)
    };
    //dbg!(&ship);
    log::info!("input_api_server ship read ok");
    let center_waterline = async {
        println!("2");
        CenterWaterlineArray::parse(&async_query(
            db_name,
            format!(
                "SELECT key, value FROM center_waterline WHERE ship_id={};",
                ship_id
            ),
        )?)
    };
    //    dbg!(&center_waterline);
    log::info!("input_api_server center_waterline read ok");
    let center_draught_shift = async {
        println!("3");
        CenterDraughtShiftDataArray::parse(&async_query(
            db_name,
            format!(
                "SELECT key, value_x, value_y, value_z FROM center_draught WHERE ship_id={};",
                ship_id
            ),
        )?)
    };
    //  dbg!(&center_draught_shift);
    log::info!("input_api_server center_draught_shift read ok");
    let mean_draught = async {
        println!("4");
        MeanDraughtDataArray::parse(&async_query(
            db_name,
            format!(
                "SELECT key, value FROM mean_draught WHERE ship_id={};",
                ship_id
            ),
        )?)
    };
    //    dbg!(&mean_draught);
    log::info!("input_api_server mean_draught read ok");
    let rad_long = async {
        println!("5");
        RadLongDataArray::parse(&async_query(
            db_name,
            format!("SELECT key, value FROM rad_long WHERE ship_id={};", ship_id),
        )?)
    };
    //    dbg!(&rad_long);
    log::info!("input_api_server rad_long read ok");
    let frame = async {
        println!("6");
        FrameDataArray::parse(&async_query(
            db_name,
            format!(
                "SELECT index, key, value FROM frame WHERE ship_id={};",
                ship_id
            ),
        )?)
    };
    //    dbg!(&frame);
    log::info!("input_api_server frame read ok");
    let frame_area = async {
        println!("7");
        FrameAreaArray::parse(
            &async_query(
                db_name,
                format!(
                    "SELECT frame_index, key, value FROM frame_area WHERE ship_id={};",
                    ship_id
                ),
            )
            .unwrap_or((&("No frame_area data!".to_string().as_bytes())).to_vec()),
        )
    };
    //    dbg!(&frame_area);
    log::info!("input_api_server frame_area read ok");
    let load_space = async {
        println!("8");
        LoadSpaceArray::parse(&async_query(
            db_name,
            format!(
                "SELECT space_id, key, value FROM load_space WHERE ship_id={};",
                ship_id
            ),
        )?)
    };
    //    dbg!(&load_space);
    log::info!("input_api_server load_space read ok");
    let load_constant = async {
        println!("9");
        LoadConstantArray::parse(&async_query(
            db_name,
            format!(
                "SELECT frame_space_index, key, value FROM load_constant WHERE ship_id={};",
                ship_id
            ),
        )?)
    };
    //    dbg!(&load_constant);
    log::info!("input_api_server load_constant read ok");
    let tank = async {
        println!("10");
        TankDataArray::parse(&async_query(
            db_name,
            format!(
                "SELECT tank_id, key, value FROM tank WHERE ship_id={};",
                ship_id
            ),
        )?)
    };
    // dbg!(&tank);
    log::info!("input_api_server tank read ok");
    let tank_center = async {
        println!("11");
        CenterVolumeArray::parse(
            &async_query(
                db_name,
                format!(
            "SELECT tank_id, key, value_x, value_y, value_z FROM tank_center WHERE tank_id={};",
            ship_id
        ),
            )
            .unwrap_or((&("No tank_center data!".to_string().as_bytes())).to_vec()),
        )
    };
    // dbg!(&tank_center);
    log::info!("input_api_server tank_center read ok");
    let tank_inertia = async {
        println!("12");
        FreeMomentInertiaArray::parse(
            &async_query(
                db_name,
                format!(
                    "SELECT tank_id, key, value_x, value_y FROM tank_center WHERE tank_id={};",
                    ship_id
                ),
            )
            .unwrap_or((&("No tank_center data!".to_string().as_bytes())).to_vec()),
        )
    };
    //  dbg!(&tank_inertia);
    log::info!("input_api_server tank_inertia read ok");

    let (
        ship,
        center_waterline,
        center_draught_shift,
        mean_draught,
        rad_long,
        frame,
        frame_area,
        load_space,
        load_constant,
        tank,
        tank_center,
        tank_inertia,
    ) = futures::join!(
        ship,
        center_waterline,
        center_draught_shift,
        mean_draught,
        rad_long,
        frame,
        frame_area,
        load_space,
        load_constant,
        tank,
        tank_center,
        tank_inertia,
    );

    log::info!("input_api_server read ok");
    ParsedShipData::parse(
        ship_id,
        ship?,
        center_waterline?,
        rad_long?,
        mean_draught?,
        center_draught_shift?,
        frame?,
        FrameAreaData::new(frame_area?.data()),
        load_constant?,
        load_space?,
        tank?,
        CenterVolumeData::new(tank_center?.data()),
        FreeMomentInertiaData::new(tank_inertia?.data()),
    )
}

/// Вспомогательная функция для выполнения запроса к апи-серверу
fn async_query(
    database: impl Into<String>,
    sql: impl Into<String>,
) -> Result<Vec<u8>, Error> {
    let query = ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(database, sql)), false);
    let mut request = ApiRequest::new(
        "parent",
        "0.0.0.0:8080",
        "auth_token",
        query.clone(),
        false,
        false,
    );
    Ok(request.fetch(&query, false)?)
}

*/