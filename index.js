const txp = import("./pkg");

const txp_demo = input_str_from_js =>
  txp.then(module => module.hello_txp(input_str_from_js)).catch(console.error);

module.exports = {
  txp_demo
};
