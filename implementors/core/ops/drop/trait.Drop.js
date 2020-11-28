(function() {var implementors = {};
implementors["backtrace"] = [{"text":"impl&lt;'_, '_, '_&gt; Drop for BacktraceFrameFmt&lt;'_, '_, '_&gt;","synthetic":false,"types":[]}];
implementors["clip_sys"] = [{"text":"impl Drop for FFICharPtr","synthetic":false,"types":[]}];
implementors["crossbeam_deque"] = [{"text":"impl&lt;T&gt; Drop for Injector&lt;T&gt;","synthetic":false,"types":[]}];
implementors["crossbeam_epoch"] = [{"text":"impl&lt;T&gt; Drop for Owned&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Drop for LocalHandle","synthetic":false,"types":[]},{"text":"impl Drop for Guard","synthetic":false,"types":[]}];
implementors["crossbeam_queue"] = [{"text":"impl&lt;T&gt; Drop for ArrayQueue&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Drop for SegQueue&lt;T&gt;","synthetic":false,"types":[]}];
implementors["crossbeam_utils"] = [{"text":"impl&lt;'a, T:&nbsp;?Sized&gt; Drop for ShardedLockWriteGuard&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl Drop for WaitGroup","synthetic":false,"types":[]}];
implementors["deflate"] = [{"text":"impl&lt;W:&nbsp;Write&gt; Drop for DeflateEncoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Write&gt; Drop for ZlibEncoder&lt;W&gt;","synthetic":false,"types":[]}];
implementors["gif"] = [{"text":"impl&lt;W:&nbsp;Write&gt; Drop for Encoder&lt;W&gt;","synthetic":false,"types":[]}];
implementors["lzw"] = [{"text":"impl&lt;W:&nbsp;BitWriter&gt; Drop for Encoder&lt;W&gt;","synthetic":false,"types":[]}];
implementors["png"] = [{"text":"impl&lt;W:&nbsp;Write&gt; Drop for Writer&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, W:&nbsp;Write&gt; Drop for StreamWriter&lt;'a, W&gt;","synthetic":false,"types":[]}];
implementors["rayon_core"] = [{"text":"impl Drop for ThreadPool","synthetic":false,"types":[]}];
implementors["scoped_threadpool"] = [{"text":"impl Drop for Pool","synthetic":false,"types":[]},{"text":"impl&lt;'pool, 'scope&gt; Drop for Scope&lt;'pool, 'scope&gt;","synthetic":false,"types":[]}];
implementors["scopeguard"] = [{"text":"impl&lt;T, F, S&gt; Drop for ScopeGuard&lt;T, F, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: FnOnce(T),<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Strategy,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl&lt;'a&gt; Drop for ParseBuffer&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["tiff"] = [{"text":"impl&lt;'a, W:&nbsp;Write + Seek&gt; Drop for DirectoryEncoder&lt;'a, W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, W:&nbsp;Write + Seek, C:&nbsp;ColorType&gt; Drop for ImageEncoder&lt;'a, W, C&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()