
## Deep link Single Instance Issue 

#### This repo is for the issue of conflict between Deep Link Plugin and Single Instance Plugin in Tauri 

### Problem: 
If we use both deep link plugin and single instance plugin and emit their event like [this](https://github.com/Astitva877/tauri-app/blob/main/src-tauri/src/main.rs#L23C13-L37C23) so the single instance plugin doesn't work, although if we only emit single instance plugin event then it works and only one instance of a window appears. 

Here is the link to the video for reference: https://drive.google.com/file/d/1zgdO9t5HwyhZwQ1Zqru3eItsEsZ2_7Li/view?usp=sharing
