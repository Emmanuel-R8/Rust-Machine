use std::path::PathBuf;

use crate::hardware::network::NetworkInterface;

#[derive(Clone, Debug)]
pub struct TraceConfig {
    pub traceP: bool,
    pub tracePOST: bool,
    pub bufferSize: u32,
    pub startPC: u32,
    pub stopPC: u32,
    pub outputFile: PathBuf,
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
    pub vlmDebuggerPath: PathBuf,
    pub worldPath: PathBuf,
    pub worldSearchPath: PathBuf,
    pub enableIDS: bool,
    pub virtualMemory: u32,
    pub coldLoadXParams: XParams,
    pub generaXParams: XParams,
    pub diagnosticIPAddress: u32,
    pub interfaces: [Option<Box<&'a NetworkInterface<'a>>>; 8],
    pub test_function: bool,
}

impl Default for VLMConfig <'static> {
    fn default() -> Self {
        let mut trace_config = TraceConfig {
            traceP: false,
            tracePOST: false,
            bufferSize: 0,
            startPC: 0,
            stopPC: 0,
            outputFile: PathBuf::from(""),
        };

        // Default configuration
        let mut config = Self {
            enable_spy: false,
            tracing: trace_config,
            comm_area_size: 0x1_FF80,
            host_buffer_space: 15_000,
            guest_buffer_space: 100_000,
            vlmDebuggerPath: PathBuf::from(""),
            worldPath: PathBuf::from("./data/world/Genera-8-5-xlib-patched.vlod"),
            worldSearchPath: PathBuf::from("."),
            enableIDS: false,
            virtualMemory: 0,
            coldLoadXParams: XParams {
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
            generaXParams: XParams {
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
            diagnosticIPAddress: 0,
            interfaces: [None, None, None, None, None, None, None, None],
            test_function: false,
        };

        // let mut options = 0 as XrmDatabase;
        let mut homeDir = PathBuf::from("");
        let mut workingDir = PathBuf::from("");
        let mut configFile = PathBuf::from("");

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
