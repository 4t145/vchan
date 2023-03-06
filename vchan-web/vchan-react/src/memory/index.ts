import { api, Board } from "../webapi"

let _boards: Board[] | null = null
export async function getBoards(reload: boolean = false): Promise<Board[]> {
    if (_boards == null || reload) {
        const resp = await api.get_board()
        if (resp.ok) {
            _boards = resp.data
            return _boards
        } else {
            throw resp.err
        }
    } else {
        return _boards
    }
}