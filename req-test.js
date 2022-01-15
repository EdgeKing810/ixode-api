import axios from 'axios';

const API_URL = 'http://127.0.0.1:8000';

axios
  .post(
    `${API_URL}/login`,
    {
      auth_data: 'EdgeKing810',
      password: 'Test123*',
    },
    {
      headers: { Authorization: `Bearer testing` },
    }
  )
  .then((res) => {
    console.log(res.data);
  });
