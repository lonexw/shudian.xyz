CREATE MIGRATION m1xs5kwl5nuahua7fzeoqps2doff3a35cksiaqczsh3arrl4kbd4pq
    ONTO m1egseqqf3rxseoi5ir6xgyqr7qyfyowdu767o7nhuva2vqyn5yqyq
{
  ALTER TYPE default::Shop {
      DROP PROPERTY tags;
  };
  CREATE TYPE default::Tag {
      CREATE LINK parent -> default::Tag;
      CREATE REQUIRED PROPERTY name -> std::str;
  };
  ALTER TYPE default::Shop {
      CREATE MULTI LINK tags -> default::Tag;
  };
};
