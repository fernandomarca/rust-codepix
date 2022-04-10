/*
  Warnings:

  - You are about to drop the column `pixKeyPId` on the `account` table. All the data in the column will be lost.
  - Added the required column `status` to the `pixkey` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_account" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "owner_name" TEXT NOT NULL,
    "bank_id" TEXT NOT NULL,
    "number" TEXT NOT NULL,
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT,
    "bankId2" TEXT NOT NULL,
    CONSTRAINT "account_bankId2_fkey" FOREIGN KEY ("bankId2") REFERENCES "bank" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_account" ("bankId2", "bank_id", "createdAt", "id", "number", "owner_name", "updatedAt") SELECT "bankId2", "bank_id", "createdAt", "id", "number", "owner_name", "updatedAt" FROM "account";
DROP TABLE "account";
ALTER TABLE "new_account" RENAME TO "account";
CREATE UNIQUE INDEX "account_id_key" ON "account"("id");
CREATE TABLE "new_pixkey" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "kind" TEXT NOT NULL,
    "key" TEXT NOT NULL,
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT,
    "accountPId" TEXT NOT NULL,
    "status" TEXT NOT NULL,
    CONSTRAINT "pixkey_accountPId_fkey" FOREIGN KEY ("accountPId") REFERENCES "account" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_pixkey" ("accountPId", "createdAt", "id", "key", "kind", "updatedAt") SELECT "accountPId", "createdAt", "id", "key", "kind", "updatedAt" FROM "pixkey";
DROP TABLE "pixkey";
ALTER TABLE "new_pixkey" RENAME TO "pixkey";
CREATE TABLE "new_transaction" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "accountPId" TEXT NOT NULL,
    "accountFromId" TEXT NOT NULL,
    "amount" REAL NOT NULL,
    "pixKeyPId" TEXT NOT NULL,
    "pixKeyIdTo" TEXT NOT NULL,
    "status" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "cancelDescription" TEXT,
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT,
    CONSTRAINT "transaction_accountPId_fkey" FOREIGN KEY ("accountPId") REFERENCES "account" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "transaction_pixKeyPId_fkey" FOREIGN KEY ("pixKeyPId") REFERENCES "pixkey" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_transaction" ("accountFromId", "accountPId", "amount", "cancelDescription", "createdAt", "description", "id", "pixKeyIdTo", "pixKeyPId", "status", "updatedAt") SELECT "accountFromId", "accountPId", "amount", "cancelDescription", "createdAt", "description", "id", "pixKeyIdTo", "pixKeyPId", "status", "updatedAt" FROM "transaction";
DROP TABLE "transaction";
ALTER TABLE "new_transaction" RENAME TO "transaction";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
