import { Nullable, None } from './types';

export const isSome = <T>(a: Nullable<T>): a is T => typeof a != null;
export const isNone = <T>(a: Nullable<T>): a is None => !isSome(a);
