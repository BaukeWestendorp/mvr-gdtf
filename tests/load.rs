use mvr_rs::MvrFile;

fn load_basic_festival() -> Result<MvrFile, mvr_rs::Error> {
    MvrFile::load_from_file("tests/mvr/basic_festival.mvr")
}

#[test]
fn test_load_basic_festival() {
    let mvr_file = load_basic_festival().unwrap();

    let gsd = mvr_file.general_scene_description();

    assert_eq!(gsd.provider(), "Provider");
    assert_eq!(gsd.provider_version(), "Provider Version");
    assert_eq!(gsd.ver_major(), 1);
    assert_eq!(gsd.ver_minor(), 5);

    let user_data = gsd.user_data().expect("UserData should exist");
    assert_eq!(
        user_data.data()[0].as_str(),
        "\r\n    <Data provider=\"Data Provider 1\" ver=\"0.1\"/>"
    );
    assert_eq!(
        user_data.data()[1].as_str(),
        "\r\n    <Data provider=\"Data Provider 2\">\r\n      <VWEntry key=\"CE7C4EDA-1C47-4B41-AF56-530116C475B2\">Custom Entry</VWEntry>\r\n    </Data>"
    );

    let aux_data = gsd.scene().aux_data().expect("AuxData should exist");
    let pos = aux_data.positions();
    assert_eq!(pos[0].name(), "Position Name 1");
    assert_eq!(
        pos[0].uuid().to_string(),
        "48444956-9b0d-11f0-a3e9-dc567b68abae"
    );
    assert_eq!(pos[1].name(), "");
    assert_eq!(
        pos[1].uuid().to_string(),
        "56b76b02-14ee-4309-bd58-0961493e93e3"
    );
    let sd = aux_data.symdefs();
    assert_eq!(sd[0].name(), "Symdef Name 1");
    assert_eq!(
        sd[0].uuid().to_string(),
        "317a5549-659d-42a8-9cdb-5e1a411560c1"
    );
    assert_eq!(matches!(
        sd[0].content(),
        SymdefContent::Geometry3D(Geometry3D)
    ));

    let layers = gsd.scene().layers();
}
