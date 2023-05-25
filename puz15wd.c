/**************************************************/
/*   �P�T�p�Y��/WD�p�e�[�u���쐬        puz15wd.c */
/*           Computer & Puzzle 2001/04 by takaken */
/**************************************************/
#include <stdio.h>
#include <stdlib.h>

#define  FALSE           0
#define  TRUE            1
#define  BOARD_WIDTH     4

#define  WDTBL_SIZE  24964 /* WalkingDistance TableSize */

typedef  unsigned __int64  u64;
typedef u64 uint64_t;

int   TABLE[BOARD_WIDTH][BOARD_WIDTH];
int   WDTOP, WDEND;
u64   WDPTN[WDTBL_SIZE];                 /* �ǖʃp�^�[�� */
char  WDTBL[WDTBL_SIZE];                 /* �ŒZ�萔(WD) */
short WDLNK[WDTBL_SIZE][2][BOARD_WIDTH]; /* �o���������N */

/*********************************************/
/* �T�����ʂ̃e�[�u�����f�B�X�N�ɕۑ�����    */
/*********************************************/
void WriteDisk(void)
{
    int  i, j, k, work[8];
    u64 table;
    char *filename = "puz15wd.db";
    FILE *fp;

    fp = fopen(filename, "wb");
    for (i=0; i<WDTBL_SIZE; i++) {
        /* WDPTN */
        table = WDPTN[i];
        for (j=7; j>=0; j--,table>>=8)
            work[j] = (int)(table & 0xff);
        for (j=0; j<8; j++)
            fputc(work[j], fp);
        /* WDTBL */
        fputc(WDTBL[i], fp);
        /* WDLNK */
        for (j=0; j<2; j++)
        for (k=0; k<4; k++) {
            fputc(WDLNK[i][j][k]  >> 8 , fp);
            fputc(WDLNK[i][j][k] & 0xff, fp);
        }
    }
    fclose(fp);
}
/*********************************************/
/* �p�^�[���̓o�^�Ƒo���������N�̌`��        */
/*********************************************/
void WriteTable(char count, int vect, int group)
{
    int  i, j, k;
    u64  table;

    /* ����p�^�[����T�� */
    table =0;
    for (i=0; i<4; i++)
    for (j=0; j<4; j++)
        table = (table << 3) | TABLE[i][j];
    for (i=0; i<WDEND; i++)
        if (WDPTN[i] == table) break;

    /* �V�K�p�^�[���o�^ */
    if (i == WDEND) {
        WDPTN[WDEND] = table;
        WDTBL[WDEND] = count;
        WDEND++;
        for (j=0; j<2; j++)
        for (k=0; k<4; k++)
            WDLNK[i][j][k] = WDTBL_SIZE;
    }

    /* �o���������N���`�������� */
    j = WDTOP - 1;
    WDLNK[j][vect    ][group] = (short)i;
    WDLNK[i][vect ^ 1][group] = (short)j;
}
/*********************************************/
/* ���D��T����WalkingDistance�����߂�       */
/*********************************************/
void Simuration(void)
{
    int  i, j, k, space=0, piece;
    char count;
    u64  table;

    /* �����ʂ���� */
    for (i=0; i<4; i++)
    for (j=0; j<4; j++)
        TABLE[i][j] = 0;
    TABLE[0][0] = TABLE[1][1] = TABLE[2][2] = 4;
    TABLE[3][3] = 3;
    table =0;
    for (i=0; i<4; i++)
    for (j=0; j<4; j++)
        table = (table << 3) | TABLE[i][j];

    /* �����ʂ�o�^ */
    WDPTN[0] = table;
    WDTBL[0] = 0;
    for (j=0; j<2; j++)
    for (k=0; k<4; k++)
        WDLNK[0][j][k] = WDTBL_SIZE;

    /* ���D��T�� */
    WDTOP=0; WDEND=1;
    while (WDTOP < WDEND) {
        /* TABLE[][]�Ăяo�� */
        table = WDPTN[WDTOP];
        count = WDTBL[WDTOP];
        WDTOP++;
        count++;

        /* TABLE[][]�Č� */
        for (i=3; i>=0; i--) {
            piece = 0;
            for (j=3; j>=0; j--) {
                TABLE[i][j] = (int)(table & 7);
                table >>= 3;
                piece += TABLE[i][j];
            }
            if (piece == 3) space = i;
        }

        /* 0:�����Ɉړ� */
        if ((piece = space + 1) < 4) {
            for (i=0; i<4; i++) {
                if (TABLE[piece][i]) {
                    TABLE[piece][i]--;
                    TABLE[space][i]++;
                    WriteTable(count, 0, i);
                    TABLE[piece][i]++;
                    TABLE[space][i]--;
                }
            }
        }

        /* 1:������Ɉړ� */
        if ((piece = space - 1) >= 0) {
            for (i=0; i<4; i++) {
                if (TABLE[piece][i]) {
                    TABLE[piece][i]--;
                    TABLE[space][i]++;
                    WriteTable(count, 1, i);
                    TABLE[piece][i]++;
                    TABLE[space][i]--;
                }
            }
        }
    }
}
int main(void)
{
    printf("making......\n");
    Simuration();
    printf("saving......\n");
    WriteDisk();
    printf("finish!\n");

    return 0;
}
