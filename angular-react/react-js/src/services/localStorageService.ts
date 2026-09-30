export function getItem<T>(name: string): T[] {
  const storageValue = localStorage.getItem(name) ?? '';
  if (storageValue.length) return JSON.parse(storageValue);
  return [];
}

export function setItem<T>(name: string, item: T[]): void {
  localStorage.setItem(name, JSON.stringify(item));
}
