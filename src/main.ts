const list = [1, 2, 3].map(x => x + 1);
console.log(list);

type Item = string | number;
const mutateItems = (items: Array<Item>): void => {
  items.push("Hello world");
}

const numbers: Array<number> = [1, 2, 3];

mutateItems(numbers);

console.log(numbers);
