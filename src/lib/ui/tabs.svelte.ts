export function createTabs(count: number) {
  let active = $state(0);

  return {
    get active() { return active; },
    set(i: number) { active = i; },
    isActive(i: number) { return active === i; },
  };
}
