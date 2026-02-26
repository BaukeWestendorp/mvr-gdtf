use facet_assert::assert_same;
use mvr_rs::{MvrFile, ScaleHandeling, SymdefChild};

fn load_basic_festival() -> Result<MvrFile, mvr_rs::Error> {
    MvrFile::load_from_file("tests/mvr/basic_festival.mvr")
}

#[test]
fn test_load_basic_festival() {
    let mvr_file = load_basic_festival().unwrap();

    let gsd = mvr_file.general_scene_description();

    assert_same!(gsd.provider(), "Provider");
    assert_same!(gsd.provider_version(), "Provider Version");
    assert_same!(gsd.ver_major(), 1);
    assert_same!(gsd.ver_minor(), 5);

    let user_data = gsd.user_data().expect("UserData should exist");
    assert_same!(
        user_data.data()[0].as_str().trim(),
        r#"<Data provider="Data Provider 1" ver="0.1" />"#
    );
    assert_same!(
        user_data.data()[1].as_str().trim(),
        r#"<Data provider="Data Provider 2"><VWEntry key="ce7c4eda-1c47-4b41-af56-530116c475b2">Custom Entry</VWEntry></Data>"#
    );

    let aux_data = gsd.scene().aux_data().expect("AuxData should exist");

    let classes = aux_data.classes();
    assert_same!(classes.len(), 1);
    assert_same!(classes[0].name(), "Class Name");
    assert_same!(
        classes[0].uuid().to_string(),
        "4157c914-094b-4808-87ee-dd7ebd6f9f97".to_string()
    );

    let pos = aux_data.positions();
    assert_same!(pos.len(), 2);
    assert_same!(pos[0].name(), "Position Name 1");
    assert_same!(
        pos[0].uuid().to_string(),
        "48444956-9b0d-11f0-a3e9-dc567b68abae".to_string()
    );
    assert_same!(pos[1].name(), "");
    assert_same!(
        pos[1].uuid().to_string(),
        "56b76b02-14ee-4309-bd58-0961493e93e3".to_string()
    );

    let sd = aux_data.symdefs();
    assert_same!(sd.len(), 4);

    // Symdef 1
    assert_same!(sd[0].name(), "Symdef Name 1");
    assert_same!(
        sd[0].uuid().to_string(),
        "317a5549-659d-42a8-9cdb-5e1a411560c1".to_string()
    );
    match sd[0].child().expect("should have child") {
        SymdefChild::Geometry3D(geom3d_0) => {
            assert_same!(
                geom3d_0.file_name(),
                "30126c7e-b5b2-49ed-b94e-d649d44af071.glb"
            );
            assert_same!(
                geom3d_0.matrix().map(|m| m.to_string()).as_deref(),
                Some("{1,2,3}{4,5,6}{7,8,9}{10,11,12}")
            );
        }
        _ => panic!("Expected Geometry3D"),
    }

    // Symdef 2
    assert_same!(sd[1].name(), "Symdef Name 2");
    assert_same!(
        sd[1].uuid().to_string(),
        "0584afe1-2cbc-4a98-b5d2-2261aafdbdbb".to_string()
    );
    match sd[1].child().expect("should have child") {
        SymdefChild::Geometry3D(geom3d_1) => {
            assert_same!(
                geom3d_1.file_name(),
                "30126c7e-b5b2-49ed-b94e-d649d44af071.glb"
            );
            assert!(geom3d_1.matrix().is_none());
        }
        _ => panic!("Expected Geometry3D"),
    }

    // Symdef 3
    assert_same!(sd[2].name(), "Symdef Name 3");
    assert_same!(
        sd[2].uuid().to_string(),
        "0f76c345-0f3f-4251-8e19-8dc0690ffd6f".to_string()
    );
    match sd[2].child().expect("should have child") {
        SymdefChild::Symbol(symbol_2) => {
            assert_same!(
                symbol_2.uuid().to_string(),
                "4de1d6e2-5437-4ec3-949e-2065cb4fbfce".to_string()
            );
            assert_same!(
                symbol_2.symdef().to_string(),
                "4dd4be9e-ba5c-4ffb-90be-0419b4d977a4".to_string()
            );
            assert!(symbol_2.matrix().is_none());
        }
        _ => panic!("Expected Symbol"),
    }

    // Symdef 4 (no name)
    assert_same!(sd[3].name(), "");
    assert_same!(
        sd[3].uuid().to_string(),
        "a1907a3e-16c1-4702-984a-9de0b41adff4".to_string()
    );
    match sd[3].child().expect("should have child") {
        SymdefChild::Symbol(symbol_3) => {
            assert_same!(
                symbol_3.uuid().to_string(),
                "f7199cb8-e6f9-493d-8d52-7cf529453fc4".to_string()
            );
            assert_same!(
                symbol_3.symdef().to_string(),
                "aa517032-d1f1-40d4-b14d-63ed6527349f".to_string()
            );
            assert_same!(
                symbol_3.matrix().map(|m| m.to_string()).as_deref(),
                Some("{1,2,3}{4,5,6}{7,8,9}{10,11,12}")
            );
        }
        _ => panic!("Expected Symbol"),
    }

    // Test MappingDefinitions
    let mappings = aux_data.mapping_definitions();
    assert_same!(mappings.len(), 1);
    assert_same!(mappings[0].name(), "Mapping Definition Name 1");
    assert_same!(
        mappings[0].uuid().to_string(),
        "bef95eb8-98ac-4217-b10d-fb4b83381398".to_string()
    );
    assert_same!(mappings[0].size_x(), 1920);
    assert_same!(mappings[0].size_y(), 1080);
    assert_same!(mappings[0].source(), Some("movie.mov"));
    assert_same!(
        mappings[0].scale_handeling(),
        ScaleHandeling::ScaleIgnoreRatio
    );

    // let layers = gsd.scene().layers();
}
