use std::path::PathBuf;

use crate::hardware::network::NetworkInterface;

#[derive(Clone, Debug)]
pub struct TraceConfig {
    pub trace_p: bool,
    pub trace_post: bool,
    pub buffer_size: u32,
    pub start_pc: u32,
    pub stop_pc: u32,
    pub output_file: PathBuf,
}

#[derive(Clone, Debug)]
pub struct XParams {
    pub xpHostName: String,
    pub xpHostAddress: u64,
    pub xpDisplay: u32,
    pub xpScreen: u32,
    pub xpInitialState: u32,
    pub xpGeometry: String,
    pub xpForegroundColor: String,
    pub xpBackgroundColor: String,
    pub xpBorderColor: String,
    pub xpBorderWidth: u32,
}

#[derive(Clone, Debug)]
pub struct VLMConfig<'a> {
    pub enable_spy: bool,
    pub tracing: TraceConfig,
    pub comm_area_size: u32,
    pub host_buffer_space: u32,
    pub guest_buffer_space: u32,
    pub vlm_debugger_path: PathBuf,
    pub world_path: PathBuf,
    pub world_search_path: PathBuf,
    pub enableIDS: bool,
    pub virtual_memory: u32,
    pub cold_load_xparams: XParams,
    pub genera_xparams: XParams,
    pub diagnostic_ipaddress: u32,
    pub interfaces: [Option<Box<&'a NetworkInterface<'a>>>; 8],
    pub test_function: bool,
}

impl Default for VLMConfig<'static> {
    fn default() -> Self {
        let trace_config = TraceConfig {
            trace_p: false,
            trace_post: false,
            buffer_size: 0,
            start_pc: 0,
            stop_pc: 0,
            output_file: PathBuf::from(""),
        };

        // Default configuration
        let config = Self {
            enable_spy: false,
            tracing: trace_config,
            comm_area_size: 0x1_ff80,
            host_buffer_space: 15_000,
            guest_buffer_space: 100_000,
            vlm_debugger_path: PathBuf::from(""),
            world_path: PathBuf::from("./data/world/Genera-8-5-xlib-patched.vlod"),
            world_search_path: PathBuf::from("."),
            enableIDS: false,
            virtual_memory: 0,
            cold_load_xparams: XParams {
                xpHostName: "".to_string(),
                xpHostAddress: 0,
                xpDisplay: 0,
                xpScreen: 0,
                xpInitialState: 0,
                xpGeometry: "".to_string(),
                xpForegroundColor: "".to_string(),
                xpBackgroundColor: "".to_string(),
                xpBorderColor: "".to_string(),
                xpBorderWidth: 0,
            },
            genera_xparams: XParams {
                xpHostName: ",".to_string(),
                xpHostAddress: 0,
                xpDisplay: 0,
                xpScreen: 0,
                xpInitialState: 0,
                xpGeometry: "".to_string(),
                xpForegroundColor: "".to_string(),
                xpBackgroundColor: "".to_string(),
                xpBorderColor: "".to_string(),
                xpBorderWidth: 0,
            },
            diagnostic_ipaddress: 0,
            interfaces: [None, None, None, None, None, None, None, None],
            test_function: false,
        };

        // let mut options = 0 as XrmDatabase;
        let mut home_dir = PathBuf::from("");
        let mut working_dir = PathBuf::from("");
        let mut config_file = PathBuf::from("");

        // XrmInitialize();
        // GetDefaultConfiguration(config, &mut options);
        // MaybeReadConfigurationFile(config, &mut options, b"VLM.conf\0" as &str);
        // homeDir = getenv(b"HOME\0");
        // if !homeDir.is_null() {
        //     sprintf(configFile.as_mut_ptr(), b"%s/.VLM\0", homeDir);
        //     MaybeReadConfigurationFile(config, &mut options, configFile.as_mut_ptr());
        // }
        // if !(getcwd(
        //     workingDir.as_mut_ptr(),
        //     ::std::mem::size_of::<[libc::c_char; 257]>(),
        // ))
        // .is_null()
        // {
        //     sprintf(
        //         configFile.as_mut_ptr(),
        //         b"%s/.VLM\0",
        //         workingDir.as_mut_ptr(),
        //     );
        //     MaybeReadConfigurationFile(config, &mut options, configFile.as_mut_ptr());
        // }
        // ProcessCommandArguments(config, &mut options, argc, argv);
        // InterpretOptions(config, options);

        return config;
    }
}

// fn get_default_configuration(mut options: *mut XrmDatabase) -> VLMConfig {
// XrmPutStringResource(
//     options,
//     b"*debugger\0",
//     b"/usr/lib/symbolics/VLM_debugger\0",
// );
// XrmPutStringResource(
//     options,
//     b"genera.world\0",
//     b"/usr/lib/symbolics/Genera-8-5.vlod\0",
// );
// XrmPutStringResource(
//     options,
//     b"minima.world\0",
//     b"/usr/lib/symbolics/Minima.mlod\0",
// );
// worldSearchPath = getenv(b"WORLDPATH\0");
// if !worldSearchPath.is_null() {
//     XrmPutStringResource(
//         options,
//         b"genera.worldSearchPath\0",
//         MergeSearchPaths(
//             worldSearchPath,
//             b"/var/lib/symbolics:/usr/lib/symbolics\0" as &str,
//         ),
//     );
// } else {
//     XrmPutStringResource(
//         options,
//         b"genera.worldSearchPath\0",
//         b"/var/lib/symbolics:/usr/lib/symbolics\0",
//     );
// }
// XrmPutStringResource(options, b"genera.enableIDS\0", b"no\0");
// XrmPutStringResource(options, b"genera.virtualMemory\0", b"200\0");
// display = getenv(b"DISPLAY\0");
// if !display.is_null() {
//     XrmPutStringResource(options, b"*display\0", display);
// } else {
//     XrmPutStringResource(options, b"*display\0", b":0.0\0");
// }
// XrmPutStringResource(options, b"*coldLoad.iconic\0", b"yes\0");

// return config;
// }
