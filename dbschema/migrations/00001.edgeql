CREATE MIGRATION m1t6avwduzctb67dktx2tyrixg5lnasnvukos2s273zqb55gkziaiq
    ONTO initial
{
  CREATE EXTENSION edgeql_http VERSION '1.0';
  CREATE TYPE default::Shop {
      CREATE REQUIRED PROPERTY address -> std::str;
      CREATE REQUIRED PROPERTY cover_image -> std::str;
      CREATE REQUIRED PROPERTY desc -> std::str;
      CREATE REQUIRED PROPERTY name -> std::str;
      CREATE REQUIRED PROPERTY open_time -> std::str;
      CREATE REQUIRED PROPERTY tags -> std::str;
      CREATE REQUIRED PROPERTY telephone -> std::str;
  };
};
