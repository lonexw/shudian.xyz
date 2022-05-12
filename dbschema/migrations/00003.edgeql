CREATE MIGRATION m1egseqqf3rxseoi5ir6xgyqr7qyfyowdu767o7nhuva2vqyn5yqyq
    ONTO m1x7szjdg2vyxjfxc2xlxdqs7f3sbkimzwuphhkmht2xwpkhl4q54a
{
  CREATE SCALAR TYPE default::ShopStatus EXTENDING enum<Auditing, Verified, Rejected>;
  ALTER TYPE default::Shop {
      CREATE REQUIRED PROPERTY status -> default::ShopStatus {
          SET default := (default::ShopStatus.Auditing);
      };
  };
};
