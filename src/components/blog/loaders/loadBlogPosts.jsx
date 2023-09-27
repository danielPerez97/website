import DaggerByExample from '../../../posts/PragmaticDaggerArticle1.md';
import PragmaticDaggerArticle2 from '../../../posts/PragmaticDaggerArticle2.md';

export async function loadBlogPosts() {
  const daggerByExampleText = (await (await fetch(DaggerByExample)).text());
  const pragmaticDaggerArticle2 = (await (await fetch(PragmaticDaggerArticle2)).text());
  // eslint-disable-next-line no-console
  const blogs = [
    {
      id: 1,
      title: 'A Pragmatic Introduction To Dagger on Android',
      shortDescription: 'Learning to use Dagger for the first time, thermosiphon and car engine free.',
      publishDate: '9/19/2023. Special thanks to Jesse Wilson for review.',
      body: daggerByExampleText,
    },
    {
      id: 2,
      title: 'A Pragmatic Introduction To Dagger on Android Part 2: Retrofit Services',
      shortDescription: 'Learn how to up Dagger 2 in an actual Android project.',
      publishDate: 'TBA',
      body: pragmaticDaggerArticle2,
    },
  ];

  return { blogs };
}

export default loadBlogPosts;
