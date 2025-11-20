import type { Ref } from "@client/models";

/**
 * Convert any object that looks like a `Ref` into a `Ref`.  If the source object
 * is `undefined`, or doesn't look like a `Ref`, return `undefined`.
 */
export const toRef = <RefLike extends Ref>(
  source: RefLike | undefined,
): Ref | undefined =>
  source?.id && source?.name ? { id: source.id, name: source.name } : undefined;

/**
 * Convert an iterable collection of `Ref`-like objects to a `Ref[]`.  Any items in the
 * collection that cannot be converted to a `Ref` will be filtered out.
 */
export const toRefs = <RefLike extends Ref>(
  source: Iterable<RefLike>,
): Array<Ref> | undefined =>
  !source
    ? undefined
    : [...source].reduce((prev, current) => {
        const curreftRef = toRef(current);
        if (curreftRef) {
          prev.push(curreftRef);
        }
        return prev;
      }, new Array<Ref>());

/**
 * Take an array of source items that look like a `Ref`, find the first one that matches
 * a given value, and return it as a `Ref`.  If no items match the value, or if the value
 * is `undefined` or `null`, then return `undefined`.
 *
 * @param items Array of source items whose first matching item will be returned as a `Ref`
 * @param itemMatchFn Function to extract data from each `item` that will be sent to `matchOperator`
 * @param matchValue The single value to match every item against
 * @param matchOperator Function to determine if `itemMatchFn` and `matchValue` match
 */
export const matchItemsToRef = <RefLike extends Ref, V>(
  items: Array<RefLike>,
  itemMatchFn: (item: RefLike) => V,
  matchValue: V | undefined | null,
  matchOperator?: (a: V, b: V) => boolean,
): Ref | undefined =>
  !matchValue
    ? undefined
    : (matchItemsToRefs(items, itemMatchFn, [matchValue], matchOperator)?.[0] ??
      undefined);

/**
 * Take an array of source items that look like a `Ref`, find the item that matches one
 * of a given array of values, and return them all as a `Ref[]`.  Any values without a
 * match will be filtered out of the resulting `Ref[]`.  If the array of values is
 * `undefined` or `null`, then return `undefined`.
 *
 * @param items Array of source items whose first matching item will be returned as a `Ref`
 * @param itemMatchFn Function to extract data from each `item` that will be sent to `matchOperator`
 * @param matchValues The array of values to match every item against
 * @param matchOperator Function to determine if `itemMatchFn` and `matchValue` match
 */
export const matchItemsToRefs = <RefLike extends Ref, V>(
  items: Array<RefLike>,
  itemMatchFn: (item: RefLike) => V,
  matchValues: Array<V | undefined | null> | undefined | null,
  matchOperator: (a: V, b: V) => boolean = (a, b) => a === b,
): Array<Ref> | undefined =>
  !matchValues
    ? undefined
    : matchValues
        .map((toMatch) =>
          !toMatch
            ? undefined
            : items.find((item) => matchOperator(itemMatchFn(item), toMatch)),
        )
        .reduce((prev, current) => {
          const curreftRef = toRef(current);
          if (curreftRef) {
            prev.push(curreftRef);
          }
          return prev;
        }, new Array<Ref>());
