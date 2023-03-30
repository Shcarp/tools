import { run } from '../../entry'
import win from '../../win'
import App from './App'
import './styles.css'

win.register({
    win_type: 'index',
    url: 'index.html'
})
run(App)
