import * as React from 'react';
import {blue} from '@mui/material/colors';
import IconButton from '@mui/material/IconButton';
import Typography from '@mui/material/Typography';
import DeleteIcon from '@mui/icons-material/Delete';
import Card from '@mui/material/Card';
import CardActions from '@mui/material/CardActions';
import CardContent from '@mui/material/CardContent';
import PropTypes from 'prop-types';
import Skeleton from '@mui/material/Skeleton';
import DialogEditTodolist from './DialogEditTodolist.js'
import { createTheme, ThemeProvider } from '@mui/material/styles';

function DeleteTodolist(id){


  const address1 = process.env.REACT_APP_API_PREFIX + "/todolists/delete/" + id; 
  const requestOptions = {
    method: "POST"
  };
  fetch(address1,  requestOptions)
    .then(
      (result) => {
      },
      (error) => {
        alert("Error=" + error);
      }
    )
}

export default function Todolist(props) {

  const theme1 = createTheme({
    typography: {
      fontFamily: 'Abril',
    },
    components: {
      MuiCssBaseline: {
        styleOverrides: `
          @font-face {
            font-family: 'Abril';
            src: local("Abril"), url('./fonts/AbrilFatface-Regular.ttf') format('truetype');
            font-weight: normal;
            font-style: normal;
          }
        `,
      },
    },
  });

  const {username, id, setUpdatedParent} = props;
  const [error, setError] = React.useState(null);
  const [items, setItems] = React.useState(null);
  const [todolist_id, setTodolistId] = React.useState("");
  const [open, setOpen] = React.useState(false);
  const [updated, setUpdated] = React.useState(true);

  const handleClose = (value) => {
    setOpen(false);
  };
 

  const deleteTodolist = (todolist_id) => {
    const theme1 = createTheme({
    typography: {
      fontFamily: 'Abril',
    },
    components: {
      MuiCssBaseline: {
        styleOverrides: `
          @font-face {
            font-family: 'Abril';
            src: local("Abril"), url('./fonts/AbrilFatface-Regular.ttf') format('truetype');
            font-weight: normal;
            font-style: normal;
          }
        `,
      },
    },
  });

    return () => {
      DeleteTodolist(todolist_id);
      setUpdatedParent((updatedParent) => !updatedParent);
    };
  };
  // Note: the empty deps array [] means
  // this useEffect will run once
  // similar to componentDidMount()
  React.useEffect(() => {
    const todolist_id = id;
    setTodolistId(todolist_id);
    console.log("todolist_id="+ todolist_id);
    setTodolistId(todolist_id);
    const address1 = process.env.REACT_APP_API_PREFIX + "/todolists/" + todolist_id;
    fetch(address1)
      .then(res => res.json())
      .then(
        (result) => {
          setItems(result.data);
        },
        // Note: it's important to handle errors here
        // instead of a catch() block so that we don't swallow
        // exceptions from actual bugs in components.
        (error) => {
          setError(error);
        }
      )
  }, [id, updated])

  DialogEditTodolist.propTypes = {
    onClose: PropTypes.func.isRequired,
    open: PropTypes.bool.isRequired,
    title: PropTypes.string.isRequired,
    priority: PropTypes.number.isRequired,
    username: PropTypes.string.isRequired,
    id: PropTypes.string.isRequired,
    setUpdated: PropTypes.func.isRequired,
  };

  if (error) {
    return <div>Error: {error.message}</div>;
  } else {
    return (
      <>
        {items === null ? (
          <Skeleton>
            <Card /> 
          </Skeleton>
        ) : (
          <Card sx={{ minWidth: 275, width: 300, margin: 0}}>
            <CardContent>
              {/*
              <Typography sx={{ fontSize: 14 }} color="text.secondary" gutterBottom>
                {items.created_at}
              </Typography>
              */}
              <ThemeProvider theme = {theme1}>
                <Typography
                  variant="h5"
                  component="div"
                  sx={{ 
                    fontSize: '1.5rem',
                    fontWeight: 600,
                    color: '#3b3b3b',
                    // letterSpacing: '.1rem',
                  }}
                >
                  {items.title}
                </Typography>
              </ThemeProvider>
              <Typography sx={{ mb: 1.5 }} color="text.secondary">
                Priority: {items.priority}
              </Typography>
            </CardContent>
            <CardActions>
              <IconButton onClick={deleteTodolist(todolist_id)} sx={{ p: 0, color: '#FF5722' }}>
                <DeleteIcon />
              </IconButton>
              <ThemeProvider theme={theme1}>
                <DialogEditTodolist 
                  onClose={handleClose}
                  open={open}
                  title={items.title}
                  priority={items.priority}
                  username={username}
                  id={todolist_id}
                  setUpdated={setUpdated}
                 
                />
              </ThemeProvider>
            </CardActions>
          </Card>
        )}
      </>
    );
  }

}
