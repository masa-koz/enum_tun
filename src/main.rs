use windows::{core::*, Win32::Foundation::*, Win32::System::Registry::*};

#[derive(Debug)]
enum WindowsDriver {
    WindowsDriverTapWindows6(String),
    WindowsDriverWintun(String),
}


fn main() -> Result<()> {
    let mut drivers: Vec<WindowsDriver> = Vec::new();

    unsafe {
        let mut adapter_key = 0;
        let ret = RegOpenKeyA(
            HKEY_LOCAL_MACHINE,
            "SYSTEM\\CurrentControlSet\\Control\\Class\\{4D36E972-E325-11CE-BFC1-08002BE10318}",
            &mut adapter_key,
        );
        if ret != ERROR_SUCCESS {
            panic!("RegOpenKeyA()={}", ret);
        }

        let mut index: u32 = 0;
        loop {
            let mut enum_name: [u8; 256] = [0; 256];
            let mut len: u32 = 256;
            let ret = RegEnumKeyExA(
                adapter_key,
                index,
                PSTR(enum_name.as_mut_ptr()),
                &mut len,
                std::ptr::null_mut(),
                PSTR(std::ptr::null_mut()),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            if ret == ERROR_NO_MORE_ITEMS {
                break;
            }
            index += 1;

            if ret != ERROR_SUCCESS {
                println!("RegEnumKeyExA({})={}", index, ret);
                continue;
            }

            let enum_name = String::from_utf8_lossy(&enum_name[..len as usize]);
            let mut unit_name = String::from("SYSTEM\\CurrentControlSet\\Control\\Class\\{4D36E972-E325-11CE-BFC1-08002BE10318}\\");
            unit_name.push_str(&enum_name);
            unit_name.push_str("\0");
            //println!("{}", unit_name);

            let mut unit_key = 0;
            let ret = RegOpenKeyA(
                HKEY_LOCAL_MACHINE,
                PSTR(unit_name.as_mut_ptr()),
                &mut unit_key,
            );
            if ret != ERROR_SUCCESS {
                println!("RegOpenKeyExA({})={}", unit_name, ret);
                continue;
            }

            let mut data_type: REG_VALUE_TYPE = 0;
            let mut component_id: [u8; 256] = [0; 256];
            let mut len: u32 = 256;
            let ret = RegQueryValueExA(
                unit_key,
                "ComponentId",
                std::ptr::null_mut(),
                &mut data_type,
                component_id.as_mut_ptr(),
                &mut len,
            );
            if ret != ERROR_SUCCESS {
                println!("RegQueryValueExA({}\\ComponentId)={}", unit_name, ret);
                continue;
            }
            let component_id = String::from_utf8_lossy(&component_id[..(len - 1) as usize]);
            //println!("{}", component_id);

            let mut data_type: REG_VALUE_TYPE = 0;
            let mut net_cfg_instance_id: [u8; 256] = [0; 256];
            let mut len: u32 = 256;
            let ret = RegQueryValueExA(
                unit_key,
                "NetCfgInstanceId",
                std::ptr::null_mut(),
                &mut data_type,
                net_cfg_instance_id.as_mut_ptr(),
                &mut len,
            );
            if ret != ERROR_SUCCESS {
                println!("RegQueryValueExA({}\\NetCfgInstanceId={}", unit_name, ret);
                continue;
            }
            let net_cfg_instance_id = String::from_utf8_lossy(&net_cfg_instance_id[..(len - 1) as usize]);
            //println!("{}", net_cfg_instance_id);

            if component_id.eq_ignore_ascii_case("Wintun") {
                drivers.push(WindowsDriver::WindowsDriverWintun(net_cfg_instance_id.to_string()));
            }
            if component_id.eq_ignore_ascii_case("tap0901") || component_id.eq_ignore_ascii_case("root\\tap0901") {
                drivers.push(WindowsDriver::WindowsDriverTapWindows6(net_cfg_instance_id.to_string()));
            }

            RegCloseKey(unit_key);
        }

        RegCloseKey(adapter_key);

        println!("{:?}", drivers);

        Ok(())
    }
}
