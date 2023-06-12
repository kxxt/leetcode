/**
 * @param {any} obj
 * @param {any} classFunction
 * @return {boolean}
 */
var checkIfInstanceOf = function (obj, classFunction) {
  return obj == null ||
    classFunction == null ||
    typeof classFunction != "function"
    ? false
    : obj instanceof classFunction ||
        (typeof obj == "string" &&
          (classFunction === String || classFunction === Object)) ||
        (typeof obj == "symbol" &&
          (classFunction === Symbol || classFunction === Object)) ||
        (typeof obj == "boolean" &&
          (classFunction === Boolean || classFunction === Object)) ||
        (typeof obj == "number" &&
          (classFunction === Number || classFunction === Object)) ||
        (typeof obj == "bigint" &&
          (classFunction === BigInt || classFunction === Object));
};

/**
 * checkIfInstanceOf(new Date(), Date); // true
 */
