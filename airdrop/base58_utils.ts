import bs58 from "bs58";
import create from "prompt-sync";

const promptUser = create();

function base58ToWallet() {
  const base58String = promptUser("Enter your base58 string: ");
//   5YnPbGxEbmxD9QJKa8DTMqewbiCDH6weBxj8EfpYN1cCmjYXi95AfuEedTuxp3uNmkenuYxHLGDhuUVhw61R5ZXZ
  const wallet: Uint8Array = bs58.decode(base58String);
  console.log(wallet);
}

function walletToBase58() {
  const wallet: Uint8Array = new Uint8Array([
    227, 121, 224, 130, 141, 107, 197, 142, 81, 28, 125, 226, 17, 202, 34, 239,
    30, 90, 101, 255, 126, 72, 251, 78, 68, 255, 167, 93, 221, 83, 162, 28, 119,
    47, 120, 16, 74, 127, 245, 160, 217, 92, 81, 159, 138, 7, 132, 72, 177, 56,
    115, 60, 211, 252, 197, 240, 27, 198, 226, 108, 145, 22, 175, 204,
  ]);
  const base58String = bs58.encode(wallet);
  console.log(base58String);
}

walletToBase58();
base58ToWallet();
// 92FVsnnubV7ZKtbqYTrkwDgvR3B8UHXT5iMGshx6XA1h
