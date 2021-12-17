import * as wasm from "../result/preset_converter.js";

wasm.greet("from Javascript");

var file;
var data = [];
data.push("This is a test\n");
data.push("Of creating a file\n");
data.push("In a browser\n");
var properties = {type: 'text/plain'}; // Specify the file's mime-type.
try {
  // Specify the filename using the File constructor, but ...
  file = new File(data, "file.txt", properties);
} catch (e) {
  // ... fall back to the Blob constructor if that isn't supported.
  file = new Blob(data, properties);
}
var url = URL.createObjectURL(file);
document.getElementById('link').href = url;
