import ccxt from "ccxt";

// Получить баланс пользователя
export async function getBalance(exchangeId: string, apiKey: string, secret: string) {
  try {
    // Проверяем, поддерживает ли ccxt биржу
    if (!ccxt.exchanges.includes(exchangeId)) {
      throw new Error(`Exchange ${exchangeId} is not supported.`);
    }

    // Получаем класс биржи
    const ExchangeClass = (ccxt as any)[exchangeId]; // Исправление!

    if (typeof ExchangeClass !== "function") {
      throw new Error(`Exchange ${exchangeId} is not a valid constructor.`);
    }

    const exchange = new ExchangeClass({
      apiKey,
      secret,
      enableRateLimit: true,
    });

    await exchange.loadMarkets();
    const balance = await exchange.fetchBalance();

    return { status: "ok", balance };
  } catch (error) {
    const err = error as Error;
    console.error(`[CCXT] Error fetching balance for ${exchangeId}:`, err.message);
    return { status: "error", message: err.message };
  }
}

// Создание ордера
export async function createTrade(
  exchangeId: string,
  apiKey: string,
  secret: string,
  symbol: string,
  side: "buy" | "sell",
  amount: number
) {
  try {
    if (!ccxt.exchanges.includes(exchangeId)) {
      throw new Error(`Exchange ${exchangeId} is not supported.`);
    }

    const ExchangeClass = (ccxt as any)[exchangeId]; // Исправление!

    if (typeof ExchangeClass !== "function") {
      throw new Error(`Exchange ${exchangeId} is not a valid constructor.`);
    }

    const exchange = new ExchangeClass({
      apiKey,
      secret,
      enableRateLimit: true,
    });

    await exchange.loadMarkets();
    const order = await exchange.createOrder(symbol, "market", side, amount);

    return { status: "ok", order };
  } catch (error) {
    const err = error as Error;
    console.error(`[CCXT] Error creating trade on ${exchangeId}:`, err.message);
    return { status: "error", message: err.message };
  }
}
