/**
 * インラインアラートメッセージの類型とテキストの構造体定義
 */
export interface InlineAlert {
  status: AlertType,
  message: string
}

export type AlertType = 'success' | 'error'
