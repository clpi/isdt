<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import { createDir, writeFile, readDir, FsTextFileOption } from '@tauri-apps/api/fs'
    var demo = {
        "name": "",
    }

    function initDemo() {
        let name = demo.name;
        let dir = "C:\\Users\\User\\Demos\\" + demo.name + "\\";
        let path = dir + name + ".yaml";
        createDir(dir);
        writeFile({
            path: path,
            contents: ""
        });
        invoke("init_demo", { name: demo.name } );
    }
    function openDemo() {
        let name = demo.name;
        let dir = "C:\\Users\\User\\Demos\\" + demo.name + "\\";
        let d = readDir(dir);
    }
    function initCapture() {
        invoke("init_capture", { name: "Capture"});
    }
    function cfgCmd() {
        invoke("cfg_cmd");
    }

</script>
<h2>New</h2>
<a href="/">&lt; Back</a>
<br/>
<input bind:value={demo.name}/><br/>
<button on:click|passive={initDemo}>initDemo</button>
<br/>
<button on:click|passive={cfgCmd}>cfgCmd</button>
<br/>
<small>Copyright 2021 Chris Pecunies</small>