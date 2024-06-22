// random generated js

const parseJson = (input) => {
  try {
    return JSON.parse(input);
  } catch (e) {
    console.log("Error parsing json: " + e);
  }
};

const parseCsv = (input) => {
  try {
    return csv.parse(input, { delimiter: "," });
  } catch (e) {
    console.log("Error parsing csv: " + e);
  }
};

const sortArray = (arr) => {
  return arr.sort((a, b) => a - b);
};

const mergeSort = (arr) => {
  if (arr.length <= 1) return arr;
  const mid = Math.floor(arr.length / 2);
  const left = mergeSort(arr.slice(0, mid));
  const right = mergeSort(arr.slice(mid));
  return merge(left, right);
};

const isArray = (obj) => {
  return Object.prototype.toString.call(obj) === "[object Array]";
};

function main() {
  console.log("Starting");
  const input = fs.readFileSync("./input.json", "utf8");
  const parsedInput = parseJson(input);
  if (isArray(parsedInput)) {
    console.log("Array");
    console.log(sortArray(parsedInput));
    console.log(mergeSort(parsedInput));
  }
  console.log(parsedInput);
}
