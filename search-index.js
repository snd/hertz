var searchIndex = {};
searchIndex["hertz"] = {"doc":"**useful functions for working with frame-rates, sample-rates,\nother rates,\ntime durations,\nfrequencies, etc and for keeping constant fps.**","items":[[5,"ns_to_s","hertz","nanoseconds to seconds",null,{"inputs":[{"name":"u64"}],"output":{"name":"f64"}}],[5,"s_to_ns","","seconds to nanoseconds",null,{"inputs":[{"name":"f64"}],"output":{"name":"f64"}}],[5,"ns_to_ms","","nanoseconds to milliseconds",null,{"inputs":[{"name":"u64"}],"output":{"name":"f64"}}],[5,"s_to_ms","","seconds to milliseconds",null,{"inputs":[{"name":"t"}],"output":{"name":"t"}}],[5,"ms_to_s","","milliseconds to seconds",null,{"inputs":[{"name":"t"}],"output":{"name":"t"}}],[5,"ms_to_ns","","milliseconds to nanoseconds",null,{"inputs":[{"name":"f64"}],"output":{"name":"f64"}}],[5,"fps_to_ns_per_frame","","when given frames per second (or sample rate)\nreturns the duration of a single frame",null,{"inputs":[{"name":"usize"}],"output":{"name":"u64"}}],[5,"current_time_ns","","returns the current timestamp in nanoseconds",null,{"inputs":[],"output":{"name":"u64"}}],[5,"sleep_for_constant_rate","","useful for keeping a constant framerate",null,{"inputs":[{"name":"usize"},{"name":"u64"}],"output":null}],[5,"hertz_range","","frequency range that can be modeled when taking the short\ntime fourier transform of a signal with `sample_rate` with\na sliding window of `window_sizew`.\nequivalent to `rayleigh(sample_rate, window_size)..nyquist(sample_rate)`\nincrease the window size to increase the lower frequency\nincrease the sample rate to increase the upper frequency",null,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"range"}}],[5,"nyquist","","maximum frequency in hertz that can be meaningfully analyzed with a given sample rate\n[https://en.wikipedia.org/wiki/Short-time_Fourier_transform#Explanation](https://en.wikipedia.org/wiki/Short-time_Fourier_transform#Explanation)",null,{"inputs":[{"name":"t"}],"output":{"name":"t"}}],[5,"rayleigh","","minimum frequency in hertz that can be meaningfully analyzed with a given sample rate and\nwindow size\n[https://en.wikipedia.org/wiki/Short-time_Fourier_transform#Rayleigh_frequency](https://en.wikipedia.org/wiki/Short-time_Fourier_transform#Rayleigh_frequency)",null,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"t"}}],[5,"cycles_per_second_to_seconds_per_cycle","","window duration in seconds\nsample_rate in hertz",null,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"t"}}]],"paths":[]};
initSearch(searchIndex);