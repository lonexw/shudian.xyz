CREATE MIGRATION m1x7szjdg2vyxjfxc2xlxdqs7f3sbkimzwuphhkmht2xwpkhl4q54a
    ONTO m1t6avwduzctb67dktx2tyrixg5lnasnvukos2s273zqb55gkziaiq
{
  CREATE TYPE default::Location {
      CREATE REQUIRED PROPERTY address -> std::str;
      CREATE REQUIRED PROPERTY city -> std::str;
      CREATE REQUIRED PROPERTY district -> std::str;
      CREATE PROPERTY latitude -> std::float32;
      CREATE PROPERTY longitude -> std::float32;
      CREATE REQUIRED PROPERTY name -> std::str;
      CREATE REQUIRED PROPERTY province -> std::str;
  };
  ALTER TYPE default::Shop {
      CREATE LINK location -> default::Location;
  };
  CREATE TYPE default::ShopSupporter {
      CREATE REQUIRED PROPERTY avatar_url -> std::str;
      CREATE REQUIRED PROPERTY joined_at -> std::datetime;
      CREATE REQUIRED PROPERTY nickname -> std::str;
      CREATE PROPERTY wx_openid -> std::str;
  };
  CREATE TYPE default::ShopPhoto {
      CREATE LINK creator -> default::ShopSupporter;
      CREATE REQUIRED PROPERTY created_at -> std::datetime;
      CREATE PROPERTY intro -> std::str;
      CREATE REQUIRED PROPERTY url -> std::str;
  };
  ALTER TYPE default::Shop {
      CREATE MULTI LINK photos -> default::ShopPhoto;
  };
  ALTER TYPE default::Shop {
      CREATE MULTI LINK supporters -> default::ShopSupporter;
  };
  ALTER TYPE default::Shop {
      ALTER PROPERTY address {
          RESET OPTIONALITY;
      };
  };
  ALTER TYPE default::Shop {
      ALTER PROPERTY cover_image {
          RENAME TO intro;
      };
  };
  ALTER TYPE default::Shop {
      ALTER PROPERTY desc {
          RENAME TO logo;
      };
  };
  ALTER TYPE default::Shop {
      CREATE PROPERTY end_at -> cal::local_date;
  };
  ALTER TYPE default::Shop {
      ALTER PROPERTY intro {
          RESET OPTIONALITY;
      };
  };
  ALTER TYPE default::Shop {
      ALTER PROPERTY logo {
          RESET OPTIONALITY;
      };
  };
  ALTER TYPE default::Shop {
      ALTER PROPERTY open_time {
          RESET OPTIONALITY;
      };
  };
  CREATE SCALAR TYPE default::OperationState EXTENDING enum<Normal, New, Close, Building>;
  ALTER TYPE default::Shop {
      CREATE PROPERTY operation_state -> default::OperationState {
          SET default := (default::OperationState.Normal);
      };
  };
  ALTER TYPE default::Shop {
      CREATE PROPERTY start_at -> cal::local_date;
  };
  ALTER TYPE default::Shop {
      ALTER PROPERTY tags {
          RESET OPTIONALITY;
      };
  };
  ALTER TYPE default::Shop {
      ALTER PROPERTY telephone {
          RESET OPTIONALITY;
      };
  };
  CREATE TYPE default::Bookshop EXTENDING default::Shop;
  CREATE TYPE default::PersonalShelf EXTENDING default::Shop;
  CREATE TYPE default::PublisingHouse EXTENDING default::Shop;
};
