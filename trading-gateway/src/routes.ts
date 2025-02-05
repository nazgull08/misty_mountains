import { Router, Request, Response } from "express";
import { getBalance, createTrade } from "./ccxt-handler";

const router = Router();

// Получить баланс
router.post("/get_balance", async (req: Request, res: Response) => {
  try {
    const { exchange, apiKey, secret } = req.body;

    if (!exchange || !apiKey || !secret) {
      res.status(400).json({ status: "error", message: "Missing required parameters." });
      return;
    }

    const result = await getBalance(exchange, apiKey, secret);
    res.json(result);
  } catch (error) {
    console.error("[ERROR] /get_balance:", error);
    res.status(500).json({ status: "error", message: "Internal server error." });
  }
});

// Совершить торговую операцию
router.post("/trade", async (req: Request, res: Response) => {
  try {
    const { exchange, apiKey, secret, symbol, side, amount } = req.body;

    if (!exchange || !apiKey || !secret || !symbol || !side || !amount) {
      res.status(400).json({ status: "error", message: "Missing required parameters." });
      return;
    }

    const result = await createTrade(exchange, apiKey, secret, symbol, side, amount);
    res.json(result);
  } catch (error) {
    console.error("[ERROR] /trade:", error);
    res.status(500).json({ status: "error", message: "Internal server error." });
  }
});

export default router;
