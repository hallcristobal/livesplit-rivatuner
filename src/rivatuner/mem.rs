use winapi::{DWORD, c_char, LARGE_INTEGER, BOOL, MAX_PATH};

#[repr(C)]
pub struct SharedMemory {
    pub signature: DWORD,
    pub version: DWORD,

    pub app_entry_size: DWORD,
    pub app_arr_offset: DWORD,
    pub app_arr_size: DWORD,

    pub osd_entry_size: DWORD,
    pub osd_arr_offset: DWORD,
    pub osd_arr_size: DWORD,

    pub osd_frame: DWORD,

    pub arr_osd: [OsdEntry; 8],
    pub arr_app: [AppEntry; 256],

    pub auto_video_capture_param: VideoCaptureParam,
}

#[repr(C)]
pub struct OsdEntry {
    pub osd: [c_char; 256],
    pub osd_owner: [c_char; 256],
    pub osd_ex: [c_char; 4096],
}

#[repr(C)]
pub struct AppEntry {
    pub process_id: DWORD,
    pub name: [c_char; MAX_PATH],
    pub flags: DWORD,

    pub time0: DWORD,
    pub time1: DWORD,
    pub frames: DWORD,
    pub fame_time: DWORD,

    pub stat_flags: DWORD,
    pub stat_time0: DWORD,
    pub stat_time1: DWORD,
    pub stat_frames: DWORD,
    pub stat_count: DWORD,
    pub stat_framerate_min: DWORD,
    pub stat_framerate_avg: DWORD,
    pub stat_framerate_max: DWORD,

    pub osd_x: DWORD,
    pub osd_y: DWORD,
    pub osd_pixel: DWORD,
    pub osd_color: DWORD,
    pub osd_frame: DWORD,

    pub screen_capture_flags: DWORD,
    pub screen_capture_path: [c_char; MAX_PATH],

    pub osd_bgnd_color: DWORD,

    pub video_capture_flags: DWORD,
    pub video_capture_path: [c_char; MAX_PATH],
    pub video_framerate: DWORD,
    pub video_framesize: DWORD,
    pub video_format: DWORD,
    pub video_quality: DWORD,
    pub video_capture_threads: DWORD,

    pub screen_capture_quality: DWORD,
    pub screen_capture_threads: DWORD,

    pub audio_capture_flags: DWORD,

    pub video_capture_flags_ex: DWORD,

    pub audio_capture_flags2: DWORD,

    pub stat_frame_time_min: DWORD,
    pub stat_frame_time_avg: DWORD,
    pub stat_frame_time_max: DWORD,
    pub stat_frame_time_count: DWORD,

    pub stat_frame_time_buf: [DWORD; 1024],
    pub stat_frame_time_buf_pos: DWORD,
    pub stat_frame_time_buf_framerate: DWORD,

    pub audio_capture_ptte_event_push: LARGE_INTEGER,
    pub audio_capture_ptte_event_release: LARGE_INTEGER,

    pub audio_capture_ptte_event_push2: LARGE_INTEGER,
    pub audio_capture_ptte_event_release2: LARGE_INTEGER,

    pub prerecord_size_limit: DWORD,
    pub prerecord_time_limit: DWORD,
}

#[repr(C)]
pub struct VideoCaptureParam {
    pub version: DWORD,
    pub filename: [c_char; MAX_PATH],
    pub framerate: DWORD,
    pub framesize: DWORD,
    pub format: DWORD,
    pub quality: DWORD,
    pub threads: DWORD,
    pub capture_osd: BOOL,
    pub audio_capture_flags: DWORD,
    pub vidoe_capture_flags_ex: DWORD,
    pub audio_capture_flags2: DWORD,
    pub prerecord_size_limit: DWORD,
    pub prerecord_time_limit: DWORD,
}
