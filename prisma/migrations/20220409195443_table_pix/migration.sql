/*
  Warnings:

  - You are about to drop the column `accountId` on the `pixkey` table. All the data in the column will be lost.
  - You are about to drop the column `accountId2` on the `pixkey` table. All the data in the column will be lost.
  - You are about to alter the column `createdAt` on the `pixkey` table. The data in that column could be lost. The data in that column will be cast from `String` to `DateTime`.
  - You are about to alter the column `updatedAt` on the `pixkey` table. The data in that column could be lost. The data in that column will be cast from `String` to `DateTime`.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_pixkey" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "kind" TEXT NOT NULL,
    "key" TEXT NOT NULL,
    "createdAt" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" DATETIME NOT NULL,
    "accountPId" TEXT NOT NULL,
    CONSTRAINT "pixkey_accountPId_fkey" FOREIGN KEY ("accountPId") REFERENCES "account" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_pixkey" ("accountPId", "createdAt", "id", "key", "kind", "updatedAt") SELECT "accountPId", "createdAt", "id", "key", "kind", "updatedAt" FROM "pixkey";
DROP TABLE "pixkey";
ALTER TABLE "new_pixkey" RENAME TO "pixkey";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
