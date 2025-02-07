import { preferenceStore } from './preference';

export async function initStore() {
  await preferenceStore.start();
}
