use rand::Rng;

pub fn bestow_wisdom() -> String {
  let mut random_number = rand::thread_rng();
  return PROVERBS[random_number.gen_range(0..PROVERBS.len())].to_owned();
}

const PROVERBS: [&str; 40] = [
  "fate has us meet from a thousand miles away",
  "hold your hand and grow old with you",
  "having love, drinking water will fill you up, without love, eating food will leave you hungry",
  "love the house with its crows (on the roof)",
  "living with love is happy, but living for love is foolish",
  "a smile is the best form of make-up",
  "give a man a fish and you feed him for a day, teach a man how to fish and you feed him for a lifetime",
  "water flows in to flow out",
  "he who asks a question might be a fool for five minutes; he who doesn't ask a question remains a fool forever",
  "fallen leaves return to their roots",
  "if you don't manage a household, you wouldn't know how expensive it is",
  "cherish a broom as if it was gold",
  "practice makes perfect",
  "teachers open the doors, but you enter by yourself",
  "learn to walk before you run",
  "hearing a hundred times doesn't compare to seeing once",
  "you cannot catch tiger cubs without entering the tiger's lair",
  "when you have a lot of friends, you don't have any real friends",
  "those with the same illness commiserate with each other",
  "the good deeds pay good recompense and the evil pays back with evil",
  "those who eat with one chopstick will stay hungry",
  "a man who eats biscuits in his bed wakes up feeling terrible",
  "after three days without reading, talk becomes flavorless",
  "a book is like a garden carried in the pocket",
  "a closed mind is like a closed book; just a block of wood",
  "it's better to be without a book than to believe a book entirely",
  "a single conversation with a wise man is worth a month's study of books",
  "if a son is uneducated, his dad is to blame",
  "a jade stone is useless before it is processed; a man is good for nothing until he is educated",
  "learning is a weightless treasure you can always carry easily",
  "true knowledge is when one knows the limitations of one's knowledge",
  "one cannot refuse to eat just because there is a chance of being choked",
  "clear conscience never fears midnight knocking",
  "once bitten by a snake, he/she is scared all his/her life at the mere sight of a rope",
  "with true friends, even water drunk together is sweet enough",
  "if you want happiness for a lifetime; help someone else",
  "better the cottage where one is merry than the palace where one weeps",
  "we count our miseries carefully, and accept our blessings without much thought",
  "you won't help shoots grow by pulling them up higher",
  "patience is a bitter plant, but its fruit is sweet",
];
