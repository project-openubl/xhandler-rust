export type KeyWithValueType<T, V> = {
  [Key in keyof T]-?: T[Key] extends V ? Key : never;
}[keyof T];

export type DisallowCharacters<
  T extends string,
  TInvalidCharacter extends string
> = T extends `${string}${TInvalidCharacter}${string}` ? never : T;
