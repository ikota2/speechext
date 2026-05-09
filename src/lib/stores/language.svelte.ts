let language = $state('en');

export const languageStore = {
  get value() { return language; },
  set(lang: string) { language = lang; }
};
