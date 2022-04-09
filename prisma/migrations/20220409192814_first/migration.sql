-- CreateTable
CREATE TABLE "account" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "owner_name" TEXT NOT NULL,
    "bank_id" TEXT NOT NULL,
    "number" TEXT NOT NULL,
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT NOT NULL,
    "bankId2" TEXT NOT NULL,
    "pixKeyPId" TEXT,
    CONSTRAINT "account_bankId2_fkey" FOREIGN KEY ("bankId2") REFERENCES "bank" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "bank" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "code" TEXT NOT NULL,
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT NOT NULL
);

-- CreateTable
CREATE TABLE "pixkey" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "kind" TEXT NOT NULL,
    "key" TEXT NOT NULL,
    "accountId" TEXT NOT NULL,
    "accountId2" TEXT NOT NULL,
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT NOT NULL,
    "accountPId" TEXT NOT NULL,
    CONSTRAINT "pixkey_accountPId_fkey" FOREIGN KEY ("accountPId") REFERENCES "account" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "transaction" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "accountPId" TEXT NOT NULL,
    "accountFromId" TEXT NOT NULL,
    "amount" REAL NOT NULL,
    "pixKeyPId" TEXT NOT NULL,
    "pixKeyIdTo" TEXT NOT NULL,
    "status" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "cancelDescription" TEXT NOT NULL,
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT NOT NULL,
    CONSTRAINT "transaction_accountPId_fkey" FOREIGN KEY ("accountPId") REFERENCES "account" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "transaction_pixKeyPId_fkey" FOREIGN KEY ("pixKeyPId") REFERENCES "pixkey" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateIndex
CREATE UNIQUE INDEX "account_id_key" ON "account"("id");

-- CreateIndex
CREATE UNIQUE INDEX "bank_id_key" ON "bank"("id");

-- CreateIndex
CREATE UNIQUE INDEX "pixkey_id_key" ON "pixkey"("id");

-- CreateIndex
CREATE UNIQUE INDEX "transaction_id_key" ON "transaction"("id");
