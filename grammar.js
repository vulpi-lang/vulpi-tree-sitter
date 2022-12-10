module.exports = grammar({
  name: "vulpi",

  rules: {
    source_file: ($) => repeat($._definition),
    _definition: ($) => token("xioco"),
  },
});
