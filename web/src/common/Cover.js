import React from 'react';
import { useNavigate, useMatch } from "@reach/router";
import { makeStyles } from '@material-ui/core/styles';
import Paper from '@material-ui/core/Paper';
import Typography from '@material-ui/core/Typography';

const useStyles = makeStyles((theme) => ({
    image: {
        position: 'relative',
        paddingBottom: '141.5094339622642%',
    },
    img: {
        width: '100%',
        height: '100%',
        objectFit: 'cover',
        position: 'absolute'
    },
    imgFavorite: {
        width: '100%',
        height: '100%',
        objectFit: 'cover',
        position: 'absolute',
        filter: 'brightness(0.5)'
    },
    title: {
        position: 'absolute',
        left: 0,
        right: 0,
        bottom: 0,
        backgroundColor: 'black',
        opacity: '60%',
        color: '#ffffff',
        textOverflow: 'ellipsis',
        whiteSpace: 'nowrap',
        overflow: 'hidden'
    }
}));

function Cover(props) {
    const classes = useStyles();

    let timeout;
    const browseMatch = useMatch('/browse/*');

    const navigate = useNavigate();
    const [favorite, setIsFavorite] = React.useState(props.isFavorite);

    const toggleFavorite = () => {
        fetch(`/api/library/manga/${props.id}`, {
            method: !favorite ? "POST" : "DELETE"
        })
            .then((response) => setIsFavorite(!favorite))
            .catch((e) => {
                console.log(e);
            });
    }

    const startTimer = () => {
        timeout = setTimeout(ontimer, 1000);
    }

    const ontimer = (e) => {
        timeout = undefined;
        toggleFavorite();
    }

    const onmousedown = (e) => {
        e.preventDefault();
        startTimer();
    }

    const onmouseup = (e) => {
        e.preventDefault();
        if (timeout) {
            clearTimeout(timeout);
            timeout = undefined;
            navigate(`/manga/${props.id}`)
        }
    }

    const ontouchstart = (e) => {
        e.preventDefault();
        startTimer();

    }

    const ontouchmove = (e) => {
        e.preventDefault();
        if (timeout) {
            clearTimeout(timeout);
            timeout = undefined;
        }
    }

    const ontouchend = (e) => {
        e.preventDefault();
        if (timeout) {
            clearTimeout(timeout);
            timeout = undefined;
            navigate(`/manga/${props.id}`)
        }
    }

    return (
        <Paper className={classes.image} onMouseDown={onmousedown} onMouseUp={onmouseup} onTouchStart={ontouchstart} onTouchMove={ontouchmove} onTouchEnd={ontouchend}>
            <img className={favorite && browseMatch ? classes.imgFavorite : classes.img} src={`/api/proxy?url=${props.coverUrl}`} alt=""></img>
            <Typography variant="subtitle1" className={classes.title}>
                {props.title}
            </Typography>
        </Paper>
    )
}

export default Cover;